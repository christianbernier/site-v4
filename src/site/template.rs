use std::rc::Rc;

use crate::site::header::SiteHeader;

#[derive(Debug)]
pub struct SiteTemplate {
  pub name: String,
  pub headers: Rc<[SiteHeader]>,
  pub body: String,
}
