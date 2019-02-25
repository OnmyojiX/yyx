use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use yyx_data::load_last_snapshot;
use yyx_types::Snapshot;

use crate::result::*;

pub struct SelectedSnapshot(RwLock<Option<Arc<Snapshot>>>);

impl SelectedSnapshot {
  pub fn new() -> Self {
    let last_snapshot = match load_last_snapshot() {
      Ok(v) => v,
      Err(err) => {
        error!("load last snapshot error: {}", err);
        None
      }
    };

    SelectedSnapshot(RwLock::new(last_snapshot.map(Arc::new)))
  }

  pub fn set(&self, snapshot: Snapshot) {
    *self.0.write().unwrap() = Some(Arc::new(snapshot))
  }
}

pub struct SnapshotRef(Arc<Snapshot>);

impl Deref for SnapshotRef {
  type Target = Snapshot;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for SnapshotRef {
  type Error = YyxError;

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    use rocket::Outcome;
    request
      .guard::<State<SelectedSnapshot>>()
      .map_failure(|_| unreachable!())
      .and_then(|selected| match *selected.0.read().unwrap() {
        Some(ref v) => Outcome::Success(SnapshotRef(v.clone())),
        None => {
          error!("Snapshot not selected: uri = {}", request.uri());
          Outcome::Failure((
            Status::BadRequest,
            YyxError::bad_request("Snapshot not selected."),
          ))
        }
      })
  }
}
