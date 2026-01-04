use color_eyre::Result;

pub mod md;
pub mod zola;

pub trait Renderer {
    fn render_header(&self, content: &str, level: usize) -> String;
    fn render_front_matter(&self, title: Option<&str>) -> String;
    fn render_reference(&self, display_text: Option<String>, target: String) -> Result<String>;
}

impl<T: Renderer + ?Sized> Renderer for &T {
    fn render_header(&self, content: &str, level: usize) -> String {
        (**self).render_header(content, level)
    }

    fn render_reference(&self, display_text: Option<String>, target: String) -> Result<String> {
        (**self).render_reference(display_text, target)
    }
    fn render_front_matter(&self, title: Option<&str>) -> String {
        (**self).render_front_matter(title)
    }
}

impl Renderer for Box<dyn Renderer> {
    fn render_header(&self, content: &str, level: usize) -> String {
        (**self).render_header(content, level)
    }
    fn render_front_matter(&self, title: Option<&str>) -> String {
        (**self).render_front_matter(title)
    }
    fn render_reference(&self, display_text: Option<String>, target: String) -> Result<String> {
        (**self).render_reference(display_text, target)
    }
}
