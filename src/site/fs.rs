use std::{fs, io, path::{Path, PathBuf}, rc::Rc};

use crate::site::{component::SiteComponent, parser::SiteParser, template::SiteTemplate};

pub struct SiteFs;
impl SiteFs {

  // from https://stackoverflow.com/a/76820878
  fn read_all_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = Self::read_all_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
  }

  pub fn read_templates(dir: &str, components: Rc<[SiteComponent]>) -> Rc<[SiteTemplate]> {
    Self::read_all_files(String::from(dir))
        .expect("Could not read files from directory.")
        .iter()
        .map(|path| {
            SiteParser::parse_template_from_string(
                &fs::read_to_string(&path)?,
                &path.file_name().expect("Page does not have filename.").to_string_lossy(),
                components.clone()
            )
        })
        .filter_map(|res| res.ok())
        .collect()
  }

  pub fn read_components(dir: &str) -> Rc<[SiteComponent]> {
      Self::read_all_files(String::from(dir))
          .expect("Could not read files from directory.")
          .iter()
          .map(|e| {
              Ok(SiteComponent {
                  name: String::from(e.file_name().expect("Component does not have filename.").to_string_lossy().trim_end_matches(".html")),
                  body: fs::read_to_string(e)?.lines().map(|l| l.trim()).collect()
              }) as io::Result<SiteComponent>
          })
          .filter_map(|res| res.ok())
          .collect()
  }

  pub fn delete_dir_if_exists(path: impl AsRef<Path>) -> io::Result<()> {
    if fs::exists(&path)? {
        fs::remove_dir_all(&path)?
    }
    Ok(())
  }

  // from https://stackoverflow.com/a/65192210
  pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
      fs::create_dir_all(&dst)?;
      for entry in fs::read_dir(src)? {
          let entry = entry?;
          let ty = entry.file_type()?;
          if ty.is_dir() {
              Self::copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
          } else {
              fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
          }
      }
      Ok(())
  }
}