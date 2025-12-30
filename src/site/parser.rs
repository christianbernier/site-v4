use std::{io, rc::Rc};

use regex::Regex;

use crate::site::{component::SiteComponent, header::SiteHeader, template::SiteTemplate};

pub struct SiteParser;
impl SiteParser {
  fn header_regex() -> Regex {
    Regex::new(r"\[([a-z_]+)=([^\[\]]+)\]").unwrap()
  }

  fn component_regex() -> Regex {
    Regex::new(r"\[\[([a-z0-9_]+)\]\]((?:[[:space:]]\[[a-z0-9_]+=[^\[\]]+\])+)").unwrap()
  }

  fn parse_headers_from_string(content: &str) -> Rc<[SiteHeader]> {
    SiteParser::header_regex().captures_iter(&content)
      .map(|c| c.extract())
      .map(|(_, [header_name, header_value])| SiteHeader {
        name: String::from(header_name),
        value: String::from(header_value)
      })
      .collect()
  }

  fn generate_component(components: Rc<[SiteComponent]>, component_name: &str, headers: Rc<[SiteHeader]>) -> String {
    let mut component_body = components
      .iter()
      .find(|c| c.name == component_name)
      .expect(&format!("No component with name \"{}\" found", component_name))
      .body
      .clone();

    headers.iter().for_each(|h| {
      component_body = component_body.replace(
        &format!("{{{{ {} }}}}", h.name),
        &h.value
      );
    });

    component_body
  }

  fn parse_body_from_string(content: &str, components: Rc<[SiteComponent]>) -> String {
    let mut body = String::from(content);

    while let Some(captures) = SiteParser::component_regex().captures(&body) {
      let component_name = captures.get(1).expect("No component name found").as_str();
      let component_headers = SiteParser::parse_headers_from_string(captures.get(2).expect("No component headers found").as_str());

      body.replace_range(
        captures.get_match().range(),
        &SiteParser::generate_component(components.clone(), component_name, component_headers)
      );
    }

    body
  }

  pub fn parse_template_from_string(content: &str, name: &str, components: Rc<[SiteComponent]>) -> io::Result<SiteTemplate> {
    let mut file_contents = content.split("---");
    let file_headers = file_contents.next().expect("File does not have file headers");
    let file_body = file_contents.next().expect("File does not have contents.");

    return Ok(SiteTemplate {
      name: String::from(name),
      headers: SiteParser::parse_headers_from_string(&file_headers),
      body: SiteParser::parse_body_from_string(&file_body, components),
    });
  }
}
