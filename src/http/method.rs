use std::str::FromStr;

use crate::http::error::HttpError;

#[derive(Debug)]
pub enum HttpMethod {
  Get,
  Post,
  Put,
  Head,
  Delete,
  Connect,
  Options,
  Trace,
  Patch
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
          HttpMethod::Get => String::from("GET"),
          HttpMethod::Post => String::from("POST"),
          HttpMethod::Put => String::from("PUT"),
          HttpMethod::Head => String::from("HEAD"),
          HttpMethod::Delete => String::from("DELETE"),
          HttpMethod::Connect => String::from("CONNECT"),
          HttpMethod::Options => String::from("OPTIONS"),
          HttpMethod::Trace => String::from("TRACE"),
          HttpMethod::Patch => String::from("PATCH"),
        }
    }
}

impl FromStr for HttpMethod {
  type Err = HttpError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "GET" => Ok(HttpMethod::Get),
        "POST" => Ok(HttpMethod::Post),
        "PUT" => Ok(HttpMethod::Put),
        "HEAD" => Ok(HttpMethod::Head),
        "DELETE" => Ok(HttpMethod::Delete),
        "CONNECT" => Ok(HttpMethod::Connect),
        "OPTIONS" => Ok(HttpMethod::Options),
        "TRACE" => Ok(HttpMethod::Trace),
        "PATCH" => Ok(HttpMethod::Patch),
        _ => Err(HttpError::InvalidMethod)
      }
  }
}
