#[derive(Debug, Clone)]
pub struct HttpHeader {
  pub name: String,
  pub value: String,
}

impl HttpHeader {
  pub fn new(name: &str, value: &str) -> HttpHeader {
    HttpHeader {
      name: String::from(name),
      value: String::from(value),
    }
  }

  pub fn get_header(headers: &Vec<HttpHeader>, header_name: &str) -> Option<String> {
    headers
      .iter()
      .find(|h| h.name.to_lowercase() == header_name.to_lowercase())
      .map(|h| h.value.clone())
  }
}
