#[derive(Fail, Debug)]
pub enum DataError {
  #[fail(display = "io error: {:?}", _0)]
  Io(::std::io::Error),

  #[fail(display = "json error: {:?}", _0)]
  Json(::serde_json::Error),

  #[fail(display = "illegal account id")]
  IllegalAccountId,

  #[fail(display = "illegal path")]
  IllegalPath,

  #[fail(display = "db error: {}", _0)]
  Db(diesel::result::Error),

  #[fail(display = "db connection error: {}", _0)]
  DbConnection(diesel::result::ConnectionError),

  #[fail(display = "db migration error: {}", _0)]
  DbMigration(diesel_migrations::RunMigrationsError),
}

impl_err_from! {
  DataError [
    (std::io::Error => |e| DataError::Io(e)),
    (serde_json::Error => |e| DataError::Json(e)),
    (diesel::result::Error => |e| DataError::Db(e)),
    (diesel_migrations::RunMigrationsError => |e| DataError::DbMigration(e))
  ]
}

pub type DataResult<T> = Result<T, DataError>;
