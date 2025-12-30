use std::str::FromStr;

use crate::http::error::HttpError;

#[derive(Debug, Clone)]
pub enum HttpProtocol {
  Http10,
  Http11
}

impl ToString for HttpProtocol {
    fn to_string(&self) -> String {
        match self {
          HttpProtocol::Http10 => String::from("HTTP/1.0"),
          HttpProtocol::Http11 => String::from("HTTP/1.1"),
        }
    }
}

impl FromStr for HttpProtocol {
  type Err = HttpError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "HTTP/1.0" => Ok(HttpProtocol::Http10),
        "HTTP/1.1" => Ok(HttpProtocol::Http11),
        _ => Err(HttpError::InvalidProtocol)
      }
  }
}
