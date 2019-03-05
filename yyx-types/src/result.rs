#[derive(Fail, Debug)]
pub enum TypesError {
  #[fail(display = "json error: {:?}", _0)]
  Json(::serde_json::Error),

  #[fail(display = "Invalid snapshot")]
  InvalidSnapshot,

  #[fail(display = "Invalid snapshot version: {}", _0)]
  InvalidVersion(String),
}

impl_err_from! {
  TypesError [
    (::serde_json::Error => |e| TypesError::Json(e))
  ]
}

pub type TypesResult<T> = Result<T, TypesError>;
