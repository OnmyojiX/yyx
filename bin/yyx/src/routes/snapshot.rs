use std::sync::Arc;

use crate::helpers::*;
use crate::result::*;

use yyx_data::save_last_snapshot;
use yyx_types::Snapshot;

#[get("/snapshot")]
pub fn get(snapshot: Option<SnapshotRef>) -> YyxResult<Json<Option<Arc<Snapshot>>>> {
  Ok(Json(snapshot.map(SnapshotRef::into_inner)))
}

#[put("/snapshot", data = "<body>")]
pub fn set(
  body: Result<Json<Snapshot>, JsonError>,
  state: State<SelectedSnapshot>,
) -> YyxResult<()> {
  let body = body.map_err(|err| match err {
    JsonError::Io(err) => YyxError::internal(format!("传输文件出错: {:?}", err)),
    JsonError::Parse(_, err) => YyxError::bad_request(format!("文件格式错误: {:?}", err)),
  })?;
  save_last_snapshot(&body)?;
  state.set(body.into_inner());
  Ok(())
}
