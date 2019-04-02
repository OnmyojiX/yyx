use crate::helpers::*;
use crate::result::*;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use serde_json::{self, Value};
use std::fs::File;
use std::path::{Path, PathBuf};
use yyx_data::save_exported_file;

#[post("/export/json?<filename>", data = "<value>")]
pub fn export_json(filename: &RawStr, value: Json<Value>) -> YyxResult<String> {
  let pretty_json = serde_json::to_string_pretty(&value as &Value)
    .map_err(|err| YyxError::bad_request(format!("JSON格式错误: {}", err)))?;
  let name = filename.percent_decode_lossy();
  let name = save_exported_file(&name, &pretty_json)?;
  Ok(name)
}

#[get("/export-files/<file..>")]
pub fn files(file: PathBuf) -> Option<Attachment<File>> {
  NamedFile::open(Path::new("exports/").join(&file))
    .ok()
    .map(move |f| {
      Attachment::new(
        file
          .file_name()
          .map(|s| s.to_string_lossy().to_string())
          .unwrap_or_else(|| "no_name".to_string()),
        f.take_file(),
      )
    })
}
