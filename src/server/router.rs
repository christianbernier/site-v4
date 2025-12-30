use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::fs;

use crate::server::error::ServerError;

#[derive(Debug)]
pub struct Route {
  resource: String,
  file: String,
}

#[derive(Debug)]
pub struct Router {
  dist_path: String,
  routes: Rc<[Route]>,
  default_file: String,
}

impl Router {
  pub fn from_file(dist_path: &str, route_file: &str, default_file: &str) -> Result<Self, ServerError> {
    if !fs::exists(Path::new(dist_path).join(default_file)).map_err(|_| {
      ServerError::CannotValidateFileExistance
    })? {
      return Err(ServerError::DefaultFileDoesNotExist);
    }

    Ok(Router {
      dist_path: String::from(dist_path),
      routes: fs::read_to_string(Path::new(route_file)).map_err(|_| ServerError::RouteFileDoesNotExist)?
        .lines()
        .filter(|l| !l.trim().starts_with("#") && l.trim().len() != 0)
        .map(|route_line| {
          let mut parts = route_line.split(" ");
          Route {
            resource: String::from(parts.next().expect(&format!("No resource found for route: {}", route_line))),
            file: String::from(parts.next().expect(&format!("No file found for route: {}", route_line))),
          }
        })
        .collect(),
      default_file: String::from(default_file),
    })
  }

  pub fn resolve_resource_filepath(&self, resource: &str) -> (PathBuf, bool) {
    match self.routes
      .iter()
      .find(|route| route.resource == resource)
      .map(|route| &route.file) {
        Some(filepath) => (Path::new(&self.dist_path).join(filepath), false),
        None => (Path::new(&self.dist_path).join(&self.default_file), true)
      }
  }
}
