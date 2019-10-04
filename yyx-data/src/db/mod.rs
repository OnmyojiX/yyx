use diesel::connection::Connection;
use diesel::sqlite::SqliteConnection;
use std::sync::{Arc, Mutex};

use crate::result::*;

pub mod player;

pub type Conn = SqliteConnection;

#[derive(Clone)]
pub struct DbRef(Arc<Mutex<SqliteConnection>>);

impl DbRef {
  pub fn new() -> DataResult<Self> {
    let path = crate::get_data_path("yyx.db")?;
    let conn =
      SqliteConnection::establish(&path.to_string_lossy()).map_err(DataError::DbConnection)?;
    Ok(DbRef(Arc::new(Mutex::new(conn))))
  }

  pub fn exec<F, T, E>(&self, f: F) -> Result<T, E>
  where
    F: FnOnce(&Conn) -> Result<T, E> + 'static,
  {
    let lock = self.0.lock().unwrap();
    f(&lock)
  }
}
