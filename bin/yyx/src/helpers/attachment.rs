use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response, Stream};
use std::io::Read;

pub struct Attachment<T: Read> {
  file_name: String,
  stream: Stream<T>,
}
impl<'r, T: Read + 'r> Responder<'r> for Attachment<T> {
  fn respond_to(self, req: &Request) -> response::Result<'r> {
    use rocket::http::hyper::header::{ContentDisposition, DispositionParam, DispositionType};
    let file_name = self.file_name.clone();
    self.stream.respond_to(req).map(move |res| {
      Response::build_from(res)
        .header(ContentDisposition {
          disposition: DispositionType::Attachment,
          parameters: vec![DispositionParam::Ext(
            "filename*".to_string(),
            format!("utf-8''{}", file_name),
          )],
        })
        .header(ContentType::Binary)
        .finalize()
    })
  }
}

impl<T: Read> Attachment<T> {
  pub fn new<S: AsRef<str>>(name: S, r: T) -> Self {
    Attachment {
      file_name: name.as_ref().to_string(),
      stream: Stream::from(r),
    }
  }
}
