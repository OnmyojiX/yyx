use serde_json::json;
use std::fmt::{self, Display};
use warp::{http::StatusCode, Rejection, Reply};

use yyx_data::result::DataError;

#[derive(Debug)]
pub struct YyxError {
  status: StatusCode,
  message: String,
}

impl fmt::Display for YyxError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for YyxError {}

impl YyxError {
  pub fn into_rejection(self) -> Rejection {
    self.into()
  }
}

impl Into<Rejection> for YyxError {
  fn into(self) -> Rejection {
    warp::reject::custom(self)
  }
}

impl YyxError {
  pub(crate) fn internal<E>(err: E) -> Self
  where
    E: Display + 'static,
  {
    YyxError {
      status: StatusCode::INTERNAL_SERVER_ERROR,
      message: err.to_string(),
    }
  }

  pub fn bad_request<E>(err: E) -> Self
  where
    E: Display + 'static,
  {
    YyxError {
      status: StatusCode::BAD_REQUEST,
      message: err.to_string(),
    }
  }
}

impl From<DataError> for YyxError {
  fn from(err: DataError) -> Self {
    YyxError::internal(err)
  }
}

pub fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(err) = err.find_cause::<YyxError>() {
    let json = warp::reply::json(&json!({
      "message": err.message.clone()
    }));
    Ok(warp::reply::with_status(json, err.status))
  } else if let Some(err) = err.find_cause::<warp::filters::body::BodyDeserializeError>() {
    let json = warp::reply::json(&json!({ "message": format!("JSON解析错误: {}", err) }));
    Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST))
  } else {
    // Could be a NOT_FOUND, or any other internal error... here we just
    // let warp use its default rendering.
    Err(err)
  }
}
