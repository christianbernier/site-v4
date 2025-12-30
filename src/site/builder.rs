use std::{fs, io::{self, Write}, path::Path, rc::Rc};

use crate::site::{fs::SiteFs, header::SiteHeader, template::SiteTemplate};

pub struct SiteBuilder;
impl SiteBuilder {
  pub fn build_template_body(headers: Rc<[SiteHeader]>, body: &str, templates: Rc<[SiteTemplate]>) -> String {
    headers.iter()
        .find(|h| h.name == "template")
        .and_then(|h| {
            let parent_template = templates.iter()
                .find(|t| t.name == h.value)
                .expect(&format!("Cannot find template: {}", h.value));

            let new_body = Self::build_template_body(
                parent_template.headers.clone(), 
                &parent_template.body, 
                templates.clone()
            ).replace("{{ body }}", body);

            Some(headers.iter()
                .map(|h| (format!("{{{{ {} }}}}", h.name), h.value.to_string()))
                .fold(new_body, |acc, (find, replace)| acc.replace(&find, &replace)))
        })
        .unwrap_or(body.to_string())
  }

  fn minify_body(original: &str) -> String {
    original.lines()
        .map(|line| line.trim())
        .collect()
  }

  pub fn compile_site() -> io::Result<()> {
    SiteFs::delete_dir_if_exists(Path::new("dist/"))?;
    SiteFs::copy_dir_all(Path::new("content/static/"), Path::new("dist/"))?;
    let components = SiteFs::read_components("content/components");
    SiteFs::read_templates("content/pages", components.clone())
        .iter()
        .map(|template| SiteTemplate {
            name: (*template.name).to_string(),
            headers: template.headers.clone(),
            body: SiteBuilder::build_template_body(
                template.headers.clone(), 
                &template.body, 
                SiteFs::read_templates("content/templates", components.clone())
            ),
        })
        .for_each(|page| {
            fs::File::create(Path::new("dist")
                .join(&page.name))
                .expect(&format!("Could not create file: {}", &page.name))
                .write_all(Self::minify_body(&page.body).as_bytes())
                .expect(&format!("Could not write to file: {}", &page.name));
        });

    Ok(())
  }
}
