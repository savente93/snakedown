use lazy_regex::regex_captures_iter;

#[derive(Debug, PartialEq, Eq)]
pub struct ObjectRef {
    pub fully_qualified_name: String,
    pub display_text: Option<String>,
}

impl ObjectRef {
    pub fn new(name: String, display: Option<String>) -> Self {
        Self {
            fully_qualified_name: name,
            display_text: display,
        }
    }
}

pub fn extract_object_refs(text: &str) -> Vec<ObjectRef> {
    regex_captures_iter!(r"\[\[(.*)\]\]", text)
        .filter_map(|captures| {
            let (_, [ref_text]) = captures.extract();

            let (name, display) = match ref_text.split_once("|") {
                Some((name, display)) => (name.trim(), Some(display.trim().to_string())),
                None => (ref_text.trim(), None),
            };

            if name.is_empty() {
                None
            } else {
                Some(ObjectRef::new(name.to_owned(), display))
            }
        })
        .collect()
}

#[cfg(test)]
mod test {

    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    use crate::indexing::object_ref::{ObjectRef, extract_object_refs};
    #[test]
    fn regex_test_captures() -> Result<()> {
        let test_text = r#"
    mod {test}
    [[]]
[[greeter]]
[[foo.bar]]
[[_foo.bar]]
[[_foo]]
[[_]]
[[asdf.asdf.asdf|display text]]
[[       foo.bar     ]]
[[      asdf.asdf.asdf       |     |||display text|||      ]]
idx[[foo == bar] = baz]
asdlkfj;alskdj;alsdkj
askdfjoiw3fmxj,cavuiw43i
        "#;

        let expected_refs = vec![
            ("greeter", None),
            ("foo.bar", None),
            ("_foo.bar", None),
            ("_foo", None),
            ("_", None),
            ("asdf.asdf.asdf", Some("display text".to_string())),
            ("foo.bar", None),
            ("asdf.asdf.asdf", Some("|||display text|||".to_string())),
        ]
        // just so I don't have to type out ObjectRef::new every time
        .into_iter()
        .map(|tup| ObjectRef::new(tup.0.to_owned(), tup.1))
        .collect::<Vec<ObjectRef>>();

        let found_refs = extract_object_refs(test_text);

        assert_eq!(expected_refs, found_refs);

        Ok(())
    }
}
