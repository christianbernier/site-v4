use crate::http::error::HttpError;

#[derive(Debug)]
pub enum ServerError {
  #[allow(dead_code)]
  HttpError(HttpError),
  CannotValidateFileExistance,
  DefaultFileDoesNotExist,
  RouteFileDoesNotExist,
  FileDoesNotExist,
  CouldNotReadFromTcpStream,
}

impl From<HttpError> for ServerError {
    fn from(value: HttpError) -> Self {
        Self::HttpError(value)
    }
}
