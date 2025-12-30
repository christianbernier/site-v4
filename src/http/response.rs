use std::rc::Rc;

use crate::http::{header, protocol, status};

#[derive(Debug)]
pub struct Response {
  pub protocol: protocol::HttpProtocol,
  pub status: status::HttpStatus,
  pub headers: Vec<header::HttpHeader>,
  pub body: Option<Rc<[u8]>>,
}

impl Response {
  pub fn format(&self) -> Vec<u8> {
    [format!(
      "{} {}{}\r\n\r\n",
      self.protocol.to_string(),
      self.status.to_string(),
      self.headers
        .iter()
        .map(|h| format!("\r\n{}: {}", h.name, h.value))
        .collect::<String>(),
    ).bytes().collect(),
    self.body.clone().unwrap_or(Rc::new([]))].concat()
  }
}
