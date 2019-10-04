use chrono::{DateTime, Local, NaiveDateTime};
use diesel::prelude::*;
use serde_derive::Serialize;

use yyx_types::Snapshot;

use crate::account_id::AccountId;
use crate::result::*;

use crate::db::Conn;
use crate::schema::player::{self, dsl};

#[derive(Queryable)]
struct Row {
  id: Option<i32>,
  server_id: i32,
  short_id: i32,
  cbg_server_id: Option<String>,
  cbg_order_sn: Option<String>,
  name: String,
  level: i32,
  latest_snapshot_date: NaiveDateTime,
  created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "player"]
struct Insert<'a> {
  server_id: i32,
  short_id: i32,
  cbg_server_id: Option<&'a str>,
  cbg_order_sn: Option<&'a str>,
  name: &'a str,
  level: i32,
  latest_snapshot_date: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Player {
  pub id: AccountId,
  pub local_id: i32,
  pub name: String,
  pub level: i32,
  pub latest_snapshot_date: NaiveDateTime,
  pub created_at: NaiveDateTime,
}

impl From<Row> for Player {
  fn from(row: Row) -> Player {
    let id = match (row.cbg_server_id, row.cbg_order_sn) {
      (Some(server_id), Some(order_sn)) => AccountId::Cbg {
        server_id,
        order_sn,
      },
      _ => AccountId::Yyx {
        server_id: row.server_id as i64,
        player_id: row.short_id as i64,
      },
    };
    Player {
      id,
      local_id: row.id.unwrap_or_default(),
      name: row.name,
      level: row.level,
      latest_snapshot_date: row.latest_snapshot_date,
      created_at: row.created_at,
    }
  }
}

pub fn list(conn: &Conn) -> DataResult<Vec<Player>> {
  Ok(
    player::table
      .load::<Row>(conn)?
      .into_iter()
      .map(Into::into)
      .collect(),
  )
}

pub fn get_by_account_id(conn: &Conn, id: &AccountId) -> DataResult<Option<Player>> {
  match *id {
    AccountId::Yyx {
      server_id,
      player_id,
    } => player::table
      .filter(
        dsl::server_id
          .eq(server_id as i32)
          .and(dsl::short_id.eq(player_id as i32)),
      )
      .first::<Row>(conn)
      .optional()
      .map(|opt| opt.map(Into::into)),
    AccountId::Cbg {
      ref server_id,
      ref order_sn,
    } => player::table
      .filter(
        dsl::cbg_server_id
          .eq(server_id)
          .and(dsl::cbg_order_sn.eq(order_sn)),
      )
      .first::<Row>(conn)
      .optional()
      .map(|opt| opt.map(Into::into)),
  }
  .map_err(Into::into)
}

pub struct SnapshotInfo {
  pub name: String,
  pub level: i32,
  pub timestamp: DateTime<Local>,
}

impl<'a> From<&'a Snapshot> for SnapshotInfo {
  fn from(s: &'a Snapshot) -> Self {
    SnapshotInfo {
      name: s.data.player.name.clone(),
      level: s.data.player.level as i32,
      timestamp: s.timestamp,
    }
  }
}

pub fn upsert(conn: &Conn, id: &AccountId, snapshot: &SnapshotInfo) -> DataResult<Player> {
  let name = &snapshot.name;
  let level = snapshot.level;
  let timestamp = snapshot.timestamp.naive_utc();
  let existing = get_by_account_id(conn, id)?;
  match existing {
    Some(p) => {
      diesel::update(player::table.find(p.local_id))
        .set((
          dsl::name.eq(name),
          dsl::level.eq(level),
          dsl::latest_snapshot_date.eq(timestamp),
        ))
        .execute(conn)?;
      Ok(p)
    }
    None => {
      let insert = match *id {
        AccountId::Yyx {
          server_id,
          player_id,
        } => Insert {
          server_id: server_id as i32,
          short_id: player_id as i32,
          cbg_server_id: None,
          cbg_order_sn: None,
          name,
          level,
          latest_snapshot_date: timestamp,
        },
        AccountId::Cbg {
          ref server_id,
          ref order_sn,
        } => Insert {
          server_id: 0,
          short_id: 0,
          cbg_server_id: Some(server_id),
          cbg_order_sn: Some(order_sn),
          name,
          level,
          latest_snapshot_date: timestamp,
        },
      };
      conn.transaction(|| {
        diesel::insert_into(player::table)
          .values(&insert)
          .execute(conn)?;
        let row: Row = player::table.order(dsl::id.desc()).first(conn)?;
        Ok(row.into())
      })
    }
  }
}

pub fn delete(conn: &Conn, id: &AccountId) -> DataResult<()> {
  let existing = get_by_account_id(conn, id)?;
  let id = if let Some(player) = existing {
    player.local_id
  } else {
    return Ok(());
  };
  conn.transaction(|| {
    diesel::delete(player::table.find(id)).execute(conn)?;

    let tag_ids: Vec<i32> = crate::schema::tag::table
      .select(crate::schema::tag::dsl::id)
      .filter(crate::schema::tag::dsl::player_id.eq(id))
      .load::<Option<i32>>(conn)?
      .into_iter()
      .filter_map(|i| i)
      .collect();

    diesel::delete(
      crate::schema::tag_object::table
        .filter(crate::schema::tag_object::dsl::tag_id.eq_any(tag_ids)),
    )
    .execute(conn)?;

    macro_rules! delete_player_data {
      ($id:expr, $($schema:ident),*) => {
        $(
          diesel::delete(
            crate::schema::$schema::table
              .filter(
                crate::schema::$schema::dsl::player_id.eq($id)
              )
          ).execute(conn)?;
        )*
      }
    }

    delete_player_data!(id, equip_group, equip_ref, hero_ref, equip_preset_ref, tag);

    Ok(())
  })
}
