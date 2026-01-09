use std::path::{Path, PathBuf};

use color_eyre::Result;

pub mod md;
pub mod zola;

pub trait Renderer {
    fn render_header(&self, content: &str, level: usize) -> String;
    fn render_front_matter(&self, title: Option<&str>) -> String;
    fn render_reference(
        &self,
        display_text: Option<String>,
        target_prefix: &Path,
        target: String,
    ) -> Result<String>;

    // This is on the Renderer because it is ssg specific.
    // e.g. zola places content in the `content` folder at the site root
    // but markdown places it just wherever it is pointed.
    fn content_path(&self) -> Option<PathBuf>;
}

impl<T: Renderer + ?Sized> Renderer for &T {
    fn render_header(&self, content: &str, level: usize) -> String {
        (**self).render_header(content, level)
    }

    fn render_reference(
        &self,
        display_text: Option<String>,
        target_prefix: &Path,
        target: String,
    ) -> Result<String> {
        (**self).render_reference(display_text, target_prefix, target)
    }
    fn render_front_matter(&self, title: Option<&str>) -> String {
        (**self).render_front_matter(title)
    }

    fn content_path(&self) -> Option<PathBuf> {
        (**self).content_path()
    }
}

impl Renderer for Box<dyn Renderer> {
    fn render_header(&self, content: &str, level: usize) -> String {
        (**self).render_header(content, level)
    }
    fn render_front_matter(&self, title: Option<&str>) -> String {
        (**self).render_front_matter(title)
    }
    fn render_reference(
        &self,
        display_text: Option<String>,
        target_prefix: &Path,
        target: String,
    ) -> Result<String> {
        (**self).render_reference(display_text, target_prefix, target)
    }
    fn content_path(&self) -> Option<PathBuf> {
        (**self).content_path()
    }
}
