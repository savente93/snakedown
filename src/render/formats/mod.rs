use std::path::PathBuf;

use color_eyre::Result;
use url::Url;

pub mod md;
pub mod zola;

pub trait Renderer {
    fn render_header(&self, content: &str, level: usize) -> String;
    fn render_front_matter(&self, title: Option<&str>) -> String;
    fn render_external_ref(&self, text: String, base_url: Url, rel_url: String) -> Result<String>;
    fn render_internal_ref(&self, text: String, rel_path: PathBuf) -> Result<String>;
}

impl<T: Renderer + ?Sized> Renderer for &T {
    fn render_header(&self, content: &str, level: usize) -> String {
        (**self).render_header(content, level)
    }

    fn render_external_ref(&self, text: String, base_url: Url, rel_url: String) -> Result<String> {
        (**self).render_external_ref(text, base_url, rel_url)
    }
    fn render_front_matter(&self, title: Option<&str>) -> String {
        (**self).render_front_matter(title)
    }
    fn render_internal_ref(&self, text: String, rel_path: PathBuf) -> Result<String> {
        (**self).render_internal_ref(text, rel_path)
    }
}

impl Renderer for Box<dyn Renderer> {
    fn render_header(&self, content: &str, level: usize) -> String {
        (**self).render_header(content, level)
    }
    fn render_front_matter(&self, title: Option<&str>) -> String {
        (**self).render_front_matter(title)
    }
    fn render_external_ref(&self, text: String, base_url: Url, rel_url: String) -> Result<String> {
        (**self).render_external_ref(text, base_url, rel_url)
    }
    fn render_internal_ref(&self, text: String, rel_path: PathBuf) -> Result<String> {
        (**self).render_internal_ref(text, rel_path)
    }
}
