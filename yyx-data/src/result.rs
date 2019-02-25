#[derive(Fail, Debug)]
pub enum DataError {
  #[fail(display = "io error: {:?}", _0)]
  Io(::std::io::Error),

  #[fail(display = "json error: {:?}", _0)]
  Json(::serde_json::Error),
}

impl_err_from! {
  DataError [
    (::std::io::Error => |e| DataError::Io(e)),
    (::serde_json::Error => |e| DataError::Json(e))
  ]
}

pub type DataResult<T> = Result<T, DataError>;
