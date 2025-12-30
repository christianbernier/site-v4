#[derive(Debug)]
pub enum HttpError {
  MissingHostHeader,
  InvalidMethod,
  InvalidProtocol,
  InvalidStartLine,
}
