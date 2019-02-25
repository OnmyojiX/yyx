use rocket::http::Status;
use rocket::response::content;
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Serialize;
use serde_json;
use std::marker::PhantomData;

use crate::result::*;

pub struct JsonString<T> {
  value: String,
  phantom: PhantomData<T>,
}

impl<T> JsonString<T>
where
  T: Serialize,
{
  pub fn new(v: &T) -> YyxResult<JsonString<T>> {
    Ok(JsonString {
      value: serde_json::to_string(v).map_err(YyxError::internal)?,
      phantom: PhantomData,
    })
  }
}

impl<'r, T> Responder<'r> for JsonString<T> {
  fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
    content::Json(self.value).respond_to(request)
  }
}
