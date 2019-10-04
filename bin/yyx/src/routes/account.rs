use serde_derive::Deserialize;
use warp::{path, Filter, Rejection, Reply};

use yyx_data::{db::player, delete_files, AccountId, DbRef};

use crate::helpers::*;
use crate::result::*;
use crate::store::YyxStoreRef;

pub fn account_id() -> impl Filter<Extract = (AccountId,), Error = Rejection> + Clone {
  path!("yyx" / i64 / i64)
    .and_then(|server_id, player_id| -> Result<_, Rejection> {
      Ok(AccountId::Yyx {
        server_id,
        player_id,
      })
    })
    .or(
      path!("cbg" / String / String).and_then(|server_id, order_sn| -> Result<_, Rejection> {
        Ok(AccountId::Cbg {
          server_id,
          order_sn,
        })
      }),
    )
    .unify()
}

// pub fn account_state(
//   store: YyxStoreRef,
// ) -> impl Filter<Extract = (AccountStateRef,), Error = Rejection> + Clone {
//   account_id().and_then(move |id| {
//     store
//       .get_account(&id)
//       .ok_or_else(|| -> Rejection { YyxError::bad_request("账号不存在").into() })
//   })
// }

pub fn get(db: DbRef) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2()
    .and(path("account").and(account_id()))
    .and_then(move |id| {
      let db = db.clone();
      block(move || {
        db.exec(move |conn| player::get_by_account_id(conn, &id))
          .map_err(|err| YyxError::internal(format!("读取账号数据: {}", err)).into_rejection())
          .map(|account| warp::reply::json(&account))
      })
    })
}

pub fn list(db: DbRef) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2().and(path("account")).and_then(move || {
    let db = db.clone();
    block(move || {
      db.exec(|conn| player::list(conn))
        .map_err(|err| YyxError::internal(format!("读取账号列表失败: {}", err)).into_rejection())
        .map(|list| warp::reply::json(&list))
    })
  })
}

pub fn list_active(
  store: YyxStoreRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2().and(path("active-account")).map(move || {
    let ids = store.get_active_states();
    warp::reply::json(&ids)
  })
}

#[derive(Deserialize)]
struct CloseQuery {
  force: Option<bool>,
}

pub fn close(
  store: YyxStoreRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::put2()
    .and(path("account"))
    .and(account_id())
    .and(path("close"))
    .and(warp::filters::query::query::<CloseQuery>())
    .and_then(move |id, query: CloseQuery| {
      if store.close_account(&id, query.force.unwrap_or_default()) {
        Ok(warp::reply::reply())
      } else {
        Err::<_, Rejection>(YyxError::bad_request("账号正在进行计算，无法关闭").into_rejection())
      }
    })
}

pub fn delete(db: DbRef) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::delete2()
    .and(path("account"))
    .and(account_id())
    .and_then(move |id| {
      let db = db.clone();
      block(move || {
        delete_files(&id).map_err(|err| {
          YyxError::internal(format!("删除账号文件失败: {}", err)).into_rejection()
        })?;
        db.exec(move |conn| player::delete(conn, &id))
          .map_err(|err| {
            YyxError::internal(format!("删除账号数据失败: {}", err)).into_rejection()
          })?;
        Ok::<_, Rejection>(warp::reply::reply())
      })
    })
}
