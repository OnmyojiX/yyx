use std::path::{Path, PathBuf};
use warp::{
  http::{header, Response},
  path::FullPath,
  Filter, Rejection, Reply,
};

use yyx_resources::YyxAsset;

pub fn static_files() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::path::full()
    .and_then(|path: FullPath| {
      let path: &str = &(path.as_str())[1..];
      let file = PathBuf::from(path);
      let (path, mime) = if let Some(ext) = file.extension() {
        (
          translate_path(&file),
          get_mime_by_extension(&ext.to_string_lossy()),
        )
      } else {
        ("index.html".to_string(), mime::TEXT_HTML_UTF_8)
      };

      if let Some(data) = YyxAsset::get(&path) {
        Ok(
          Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(data)
            .map_err(warp::reject::custom)?,
        )
      } else {
        Err(warp::reject::not_found())
      }
    })
    .boxed()
}

fn get_mime_by_extension(ext: &str) -> mime::Mime {
  match ext {
    "js" => mime::APPLICATION_JAVASCRIPT_UTF_8,
    "json" => mime::APPLICATION_JSON,
    "html" => mime::TEXT_HTML_UTF_8,
    "css" => mime::TEXT_CSS_UTF_8,
    "jpg" | "jpeg" => mime::IMAGE_JPEG,
    "png" => mime::IMAGE_PNG,
    "svg" => mime::IMAGE_SVG,
    "gif" => mime::IMAGE_GIF,
    "woff" => mime::FONT_WOFF,
    "woff2" => mime::FONT_WOFF2,
    _ => mime::TEXT_PLAIN_UTF_8,
  }
}

#[cfg(not(target_os = "windows"))]
fn translate_path(path: &Path) -> String {
  path.to_string_lossy().to_string()
}

#[cfg(target_os = "windows")]
fn translate_path(path: &Path) -> String {
  let path = path.to_string_lossy();
  path.replace("\\", "/").to_string()
}
