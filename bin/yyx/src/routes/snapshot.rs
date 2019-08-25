use rocket::response::NamedFile;
use serde_derive::Deserialize;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use crate::helpers::*;
use crate::result::*;

use yyx_cbg::{result::CbgError, CbgSnapshot};
use yyx_data::{get_last_snapshot_path, save_last_snapshot};
use yyx_types::Snapshot;

#[get("/snapshot")]
pub fn get(snapshot: Option<SnapshotRef>) -> YyxResult<Json<Option<Arc<Snapshot>>>> {
  Ok(Json(snapshot.map(SnapshotRef::into_inner)))
}

#[get("/snapshot-export/<filename..>")]
pub fn export(
  filename: PathBuf,
  _snapshot: Option<SnapshotRef>,
) -> YyxResult<Option<Attachment<File>>> {
  let f = NamedFile::open(
    get_last_snapshot_path()
      .map_err(|err| YyxError::bad_request(format!("读取快照文件错误: {}", err)))?,
  )
  .ok()
  .map(move |f| Attachment::new(filename.to_string_lossy(), f.take_file()));
  Ok(f)
}

#[derive(Debug, Deserialize)]
pub struct PullCbg {
  url: String,
}

#[put("/snapshot-cbg", data = "<body>")]
pub fn pull_cbg(body: Json<PullCbg>) -> YyxResult<Json<CbgSnapshot>> {
  let snapshot =
    yyx_cbg::pull::pull(&body.url, crate::version::VERSION).map_err(|err| match err {
      CbgError::UrlPrefix(url) => YyxError::bad_request(format!("藏宝阁网址应该以'{}'开头。", url)),
      CbgError::InvalidUrl => YyxError::bad_request(format!("藏宝阁网址错误。")),
      err => YyxError::internal(format!("读取藏宝阁数据出错: {:?}", err)),
    })?;
  Ok(Json(snapshot))
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
