use crate::http::{error::HttpError, header::HttpHeader, method::HttpMethod, protocol::HttpProtocol};

#[derive(Debug)]
pub struct Metadata {
  pub method: HttpMethod,
  pub path: String,
  pub protocol: HttpProtocol,
  pub headers: Vec<HttpHeader>,
}

impl Metadata {
  pub fn validate(&self) -> Result<(), HttpError> {
    match self.protocol {
      HttpProtocol::Http10 => {
        Ok(())
      }
      HttpProtocol::Http11 => {
        HttpHeader::get_header(&self.headers, "Host")
          .map(|_| ())
          .ok_or(HttpError::MissingHostHeader)
      }
    }
  }
}
