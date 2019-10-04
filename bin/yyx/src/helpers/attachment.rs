use warp::{http::header, Reply};

pub fn get_content_disposition(file_name: &str) -> String {
  format!("attachment; filename*=utf-8''{}", file_name)
}

pub fn attachment<T>(reply: T, file_name: &str) -> impl Reply
where
  T: Reply,
{
  warp::reply::with_header(
    warp::reply::with_header(
      reply,
      header::CONTENT_DISPOSITION,
      get_content_disposition(file_name),
    ),
    header::CONTENT_TYPE,
    mime::APPLICATION_OCTET_STREAM.as_ref(),
  )
}
