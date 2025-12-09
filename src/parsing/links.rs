use color_eyre::Result;
use color_eyre::eyre::eyre;
use lazy_regex::{regex_captures, regex_captures_iter};

use crate::indexing::object_ref::ObjectRef;

/// just a reference that is used in the text
/// not necessarily valid
pub struct UserRef {
    display_text: Option<String>,
    ref_path: String,
}

pub fn extract_references(s: &str) -> Option<Vec<UserRef>> {
    let mut references = Vec::new();

    // references should look like [[ref_path|display_text]]
    // thanks to Obsidian for this idea.
    for (_, [ref_path, display_text]) in
        regex_captures_iter!(r"\[\[(.*?)(\|.*)?\]\]", &s).map(|c| c.extract())
    {
        let user_ref = UserRef {
            display_text: if display_text.is_empty() {
                None
            } else {
                Some(display_text.to_owned())
            },
            ref_path: ref_path.to_string(),
        };
        references.push(user_ref);
    }

    if references.is_empty() {
        None
    } else {
        Some(references)
    }
}
