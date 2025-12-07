use color_eyre::Result;
use color_eyre::eyre::eyre;
use lazy_regex::regex_captures;

pub struct Ref {
    display_text: String,
    ref_path: String,
}

pub fn parse_reference(s: &str) -> Result<Ref> {
    // shoulc look like [[ref_path|display_text]]
    // thanks to Obsidian for this idea.
    if let Some((_whole, ref_path, display_text)) = regex_captures!(r"\[\[(.*?)(\|.*)?\]\]", &s) {
        Ok(Ref {
            ref_path: ref_path.to_string(),
            display_text: display_text.to_string(),
        })
    } else {
        Err(eyre!("failed to parse ref"))
    }
}
