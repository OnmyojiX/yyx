//! Local YYX data

#![warn(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate yyx_utils;
#[macro_use]
extern crate diesel_migrations;

use std::fs;
use std::path::{Path, PathBuf};

use yyx_types::Snapshot;

pub mod account_id;
pub mod db;
pub mod result;

mod schema;

pub use self::account_id::AccountId;
pub use self::db::DbRef;

use self::result::*;

pub fn init() -> DataResult<()> {
  ensure_data_dir()?;

  Ok(())
}

const LAST_SNAPSHOT_FILE_NAME: &str = "last_snapshot.json";

pub fn get_last_snapshot_path(id: &AccountId) -> DataResult<PathBuf> {
  get_account_path(id, LAST_SNAPSHOT_FILE_NAME)
}

pub fn save_last_snapshot(id: &AccountId, snapshot: &Snapshot) -> DataResult<()> {
  use std::io::BufWriter;
  let file = fs::File::create(get_account_path(id, LAST_SNAPSHOT_FILE_NAME)?)?;
  serde_json::to_writer(BufWriter::new(file), snapshot)?;
  Ok(())
}

pub fn load_last_snapshot(id: &AccountId) -> DataResult<Option<Snapshot>> {
  use std::io::ErrorKind;
  fs::read(get_account_path(id, LAST_SNAPSHOT_FILE_NAME)?)
    .map(Some)
    .or_else(|err| {
      if let ErrorKind::NotFound = err.kind() {
        Ok(None)
      } else {
        Err(err.into())
      }
    })
    .and_then(|bytes| {
      if let Some(bytes) = bytes {
        serde_json::from_reader(&bytes as &[u8]).map_err(Into::into)
      } else {
        Ok(None)
      }
    })
}

pub fn delete_files(id: &AccountId) -> DataResult<()> {
  std::fs::remove_dir_all(get_account_path(id, "")?).map_err(Into::into)
}

pub fn save_exported_file<T: AsRef<[u8]>>(name: &str, data: T) -> DataResult<String> {
  let name = sanitize_name(name);
  fs::write(Path::new("exports").join(&name), data)?;
  Ok(name)
}

fn sanitize_name(name: &str) -> String {
  name
    .chars()
    .filter(|c| c.is_alphanumeric() || "._-".contains(*c))
    .collect()
}

fn ensure_data_dir() -> DataResult<()> {
  fs::create_dir_all("data").map_err(|err| {
    error!("Create data dir error: {}", err);
    err
  })?;
  fs::create_dir_all("exports").map_err(|err| {
    error!("Create exports dir error: {}", err);
    err
  })?;
  Ok(())
}

fn get_data_path<T: AsRef<Path>>(path: T) -> DataResult<PathBuf> {
  ensure_data_dir()?;
  Ok(Path::new("data").join(path))
}

fn get_account_path<T: AsRef<Path>>(id: &AccountId, rpath: T) -> DataResult<PathBuf> {
  let path: String = match *id {
    AccountId::Yyx {
      server_id,
      player_id,
    } => vec!["yyx", &server_id.to_string(), &player_id.to_string()].join("/"),
    AccountId::Cbg {
      ref server_id,
      ref order_sn,
    } => vec!["cbg", &format!("{}_{}", server_id, order_sn)].join("/"),
  };
  let dir_path = sanitize_path(get_data_path("account")?, &path)?;
  fs::create_dir_all(&dir_path).map_err(|err| {
    error!("Create account dir error: {}", err);
    err
  })?;
  Ok(dir_path.join(rpath))
}

fn sanitize_path(base: impl AsRef<Path>, p: &str) -> DataResult<PathBuf> {
  let mut buf = PathBuf::from(base.as_ref());
  for seg in p.split('/') {
    if seg.starts_with("..") {
      return Err(DataError::IllegalPath);
    } else if seg.contains('\\') {
      return Err(DataError::IllegalPath);
    } else {
      buf.push(seg);
    }
  }
  Ok(buf)
}
