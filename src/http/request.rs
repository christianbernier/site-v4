use crate::http::metadata::Metadata;

#[derive(Debug)]
pub struct Request {
  pub metadata: Metadata,

  #[allow(dead_code)] // currently not accepting any requests with data
  pub body: Option<String>,
}
