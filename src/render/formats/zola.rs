use std::path::PathBuf;

use color_eyre::Result;
use url::Url;

use crate::render::formats::Renderer;

pub struct ZolaRenderer {
    use_shortcodes: bool,
}
impl Default for ZolaRenderer {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ZolaRenderer {
    pub fn new(use_shortcodes: bool) -> Self {
        Self { use_shortcodes }
    }
}
impl Renderer for ZolaRenderer {
    fn render_header(&self, content: &str, level: usize) -> String {
        let mut out = String::new();
        out.push_str(&"#".repeat(level));
        out.push(' ');
        out.push_str(content);
        out.push('\n');
        out
    }

    fn render_front_matter(&self, title: Option<&str>) -> String {
        let mut out = String::new();
        out.push_str("+++\n");
        if let Some(t) = title {
            out.push_str(&format!("title = \"{t}\"\n"));
        };
        out.push_str("+++\n");
        out
    }

    fn render_external_ref(&self, text: String, base_url: Url, rel_url: String) -> Result<String> {
        let full_url = base_url.join(&rel_url)?;
        if self.use_shortcodes {
            Ok(format!(
                r#"{{{{ snakedown_external_ref(text="{text}", url="{full_url}") }}}}"#
            ))
        } else {
            Ok(format!("[{text}]({full_url})"))
        }
    }

    fn render_internal_ref(&self, text: String, rel_path: PathBuf) -> Result<String> {
        let path_display = rel_path.display();
        if self.use_shortcodes {
            Ok(format!(
                r#"{{{{ snakedown_internal_ref(text="{text}", path="@/{path_display}") }}}}"#
            ))
        } else {
            Ok(format!("[{text}](@/{path_display})"))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_zola_header() -> Result<()> {
        let renderer = ZolaRenderer::new(false);
        let obj_name = String::from("foo.bar.nasty-names_with_underscores_and_emoji_🙈");
        assert_eq!(
            renderer.render_header(&obj_name, 2),
            "## foo.bar.nasty-names_with_underscores_and_emoji_🙈\n"
        );
        Ok(())
    }

    #[test]
    fn test_empty_zola_front_matter() -> Result<()> {
        assert_eq!(
            ZolaRenderer::new(false).render_front_matter(None),
            r"+++
+++
"
        );
        Ok(())
    }

    #[test]
    fn zola_external_link_no_shortcode() -> Result<()> {
        assert_eq!(
            ZolaRenderer::new(false).render_external_ref(
                "Dataset".to_string(),
                Url::parse("https://docs.xarray.dev/en/stable/")?,
                "generated/xarray.Dataset.html#xarray.Dataset".to_string()
            )?,
            r#"[Dataset](https://docs.xarray.dev/en/stable/generated/xarray.Dataset.html#xarray.Dataset)"#
        );
        Ok(())
    }
    #[test]
    fn zola_external_link_with_shortcode() -> Result<()> {
        assert_eq!(
            ZolaRenderer::new(true).render_external_ref(
                "Dataset".to_string(),
                Url::parse("https://docs.xarray.dev/en/stable/")?,
                "generated/xarray.Dataset.html#xarray.Dataset".to_string()
            )?,
            r#"{{ snakedown_external_ref(text="Dataset", url="https://docs.xarray.dev/en/stable/generated/xarray.Dataset.html#xarray.Dataset") }}"#
        );
        Ok(())
    }
    #[test]
    fn test_zola_front_matter_with_title() -> Result<()> {
        assert_eq!(
            ZolaRenderer::new(false).render_front_matter(Some("foo")),
            r#"+++
title = "foo"
+++
"#
        );
        Ok(())
    }
}
