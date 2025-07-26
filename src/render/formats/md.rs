use std::path::PathBuf;

use color_eyre::Result;
use url::Url;

use crate::render::formats::Renderer;

#[derive(Default)]
pub struct MdRenderer {}
impl MdRenderer {
    pub fn new() -> Self {
        Self {}
    }
}
impl Renderer for MdRenderer {
    fn render_header(&self, content: &str, level: usize) -> String {
        let mut out = String::new();
        out.push_str(&"#".repeat(level));
        out.push(' ');
        out.push_str(content);
        out.push('\n');
        out
    }

    fn render_front_matter(&self, title: Option<&str>) -> String {
        if let Some(t) = title {
            self.render_header(t, 1)
        } else {
            String::new()
        }
    }

    fn render_external_ref(&self, text: String, base_url: Url, rel_url: String) -> Result<String> {
        let full_url = base_url.join(&rel_url)?;
        Ok(format!("[{text}]({full_url})"))
    }

    fn render_internal_ref(&self, text: String, rel_path: PathBuf) -> Result<String> {
        let display_path = rel_path.display();
        Ok(format!("[{text}]({display_path})"))
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use color_eyre::Result;
    use url::Url;

    use crate::render::formats::{Renderer, md::MdRenderer};
    #[test]
    fn test_render_md_header() -> Result<()> {
        let text = String::from("foo");
        let out = MdRenderer::new().render_header(&text, 1);
        assert_eq!(out, String::from("# foo\n"));
        Ok(())
    }

    #[test]
    fn test_render_external_ref() -> Result<()> {
        let text = String::from("foo");
        let base_url = Url::parse("https://example.com/docs/")?;
        let rel_url = String::from("foo/bar/baz.html#Bullshit");
        let out = MdRenderer::new().render_external_ref(text, base_url, rel_url)?;
        assert_eq!(
            out,
            String::from("[foo](https://example.com/docs/foo/bar/baz.html#Bullshit)")
        );
        Ok(())
    }
    #[test]
    fn test_render_internal_ref() -> Result<()> {
        let text = String::from("foo");
        let rel_path = PathBuf::from("foo/bar/index.md");

        let out = MdRenderer::new().render_internal_ref(text, rel_path)?;
        assert_eq!(out, String::from("[foo](foo/bar/index.md)"));
        Ok(())
    }
}
