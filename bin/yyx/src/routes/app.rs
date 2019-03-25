use rocket::http::{ContentType, Status};
use rocket::Response;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use yyx_resources::YyxAsset;

#[get("/")]
pub fn index<'a>() -> Response<'a> {
  let resolve =
    YyxAsset::get("index.html").map(|data| (ContentType::from_extension("html").unwrap(), data));
  if let Some((content_type, data)) = resolve {
    Response::build()
      .header(content_type)
      .sized_body(Cursor::new(data))
      .finalize()
  } else {
    Response::build().status(Status::NotFound).finalize()
  }
}

#[cfg(not(target_os = "windows"))]
fn translate_path(path: &Path) -> String {
  let path = path.to_string_lossy();
  path.to_string()
}

#[cfg(target_os = "windows")]
fn translate_path(path: &Path) -> String {
  let path = path.to_string_lossy();
  path.replace("\\", "/").to_string()
}

#[get("/<file..>")]
pub fn static_file<'a>(file: PathBuf) -> Response<'a> {
  if file.extension().is_none() {
    return index();
  }

  let path = translate_path(&file);
  let resolve = YyxAsset::get(&path).map(|data| {
    (
      file
        .extension()
        .and_then(|ext| ContentType::from_extension(&ext.to_string_lossy()))
        .unwrap_or(ContentType::Binary),
      data,
    )
  });
  if let Some((content_type, data)) = resolve {
    Response::build()
      .header(content_type)
      .sized_body(Cursor::new(data))
      .finalize()
  } else {
    Response::build().status(Status::NotFound).finalize()
  }
}
