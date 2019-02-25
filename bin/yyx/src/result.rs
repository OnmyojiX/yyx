use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use std::fmt::Display;

use yyx_data::result::DataError;

#[derive(Debug)]
pub struct YyxError {
  status: Status,
  message: String,
}

impl YyxError {
  pub(crate) fn internal<E>(err: E) -> Self
  where
    E: Display + 'static,
  {
    YyxError {
      status: Status::InternalServerError,
      message: err.to_string(),
    }
  }

  pub fn bad_request<E>(err: E) -> Self
  where
    E: Display + 'static,
  {
    YyxError {
      status: Status::BadRequest,
      message: err.to_string(),
    }
  }
}

impl<'r> Responder<'r> for YyxError {
  fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
    json!({
      "message": self.message
    })
    .respond_to(request)
    .map(|mut res| {
      res.set_status(self.status);
      res
    })
  }
}

impl From<DataError> for YyxError {
  fn from(err: DataError) -> Self {
    YyxError::internal(err)
  }
}

pub type YyxResult<T> = Result<T, YyxError>;
