#[derive(Fail, Debug)]
pub enum CbgError {
  #[fail(display = "json error: {:?}", _0)]
  Json(serde_json::Error),

  #[fail(display = "http error: {:?}", _0)]
  Http(reqwest::Error),

  #[fail(display = "parse url error: {:?}", _0)]
  ParseUrl(url::ParseError),

  #[fail(display = "parse date error: {:?}", _0)]
  ParseDate(chrono::format::ParseError),

  #[fail(display = "url should start with '{}'", _0)]
  UrlPrefix(&'static str),

  #[fail(display = "invalid CBG url")]
  InvalidUrl,

  #[fail(display = "convert: {:?}", _0)]
  Convert(crate::convert::CbgConversionError),
}

impl_err_from! {
  CbgError [
    (serde_json::Error => |e| CbgError::Json(e)),
    (reqwest::Error => |e| CbgError::Http(e)),
    (url::ParseError => |e| CbgError::ParseUrl(e)),
    (chrono::format::ParseError => |e| CbgError::ParseDate(e))
  ]
}

pub type CbgResult<T> = Result<T, CbgError>;
