pub mod args;
pub mod expr;
pub mod formats;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use strum::Display;

use args::render_args;
use expr::render_expr;

use crate::{
    parsing::{
        class::ClassDocumentation, function::FunctionDocumentation, module::ModuleDocumentation,
    },
    render::formats::Renderer,
};

#[derive(Clone, Copy, Debug, Display, ValueEnum, PartialEq, Eq, Serialize, Deserialize)]
pub enum SSG {
    Markdown,
    Zola,
}

pub fn translate_filename(path: &Path) -> PathBuf {
    let mut translated = path.with_extension("md");
    if translated.file_stem() == Some(OsStr::new("__init__")) {
        translated = translated.with_file_name("_index.md");
    }

    translated
}

pub fn render_module<R: Renderer>(mod_doc: ModuleDocumentation, renderer: &R) -> String {
    let mut out = String::new();
    let maybe_qualifier = match (&mod_doc.prefix, &mod_doc.name) {
        (None, None) => None,
        (None, Some(name)) => Some(name.to_string()),
        (Some(pref), None) => Some(pref.to_string()),
        (Some(pref), Some(name)) => Some(format!("{pref}.{name}")),
    };

    let front_matter_str = renderer.render_front_matter(maybe_qualifier.as_deref());
    if !front_matter_str.is_empty() {
        out.push_str(&front_matter_str);
    }

    if let Some(docstring) = &mod_doc.docstring {
        out.push('\n');
        out.push_str(docstring.trim());
        out.push('\n');
    }

    for fn_docs in mod_doc.functions {
        out.push('\n');
        let sub_prefix = match (&mod_doc.prefix, &mod_doc.name) {
            (None, None) => None,
            (None, Some(name)) => Some(name.clone()),
            (Some(pref), None) => Some(pref.clone()),
            (Some(pref), Some(name)) => Some(format!("{pref}.{name}")),
        };
        out.push_str(render_function_docs(fn_docs, &sub_prefix, 2, renderer).trim_end());
        out.push('\n');
    }

    for class_docs in mod_doc.classes {
        let sub_prefix = match (&mod_doc.prefix, &mod_doc.name) {
            (None, None) => None,
            (None, Some(name)) => Some(name.clone()),
            (Some(pref), None) => Some(pref.clone()),
            (Some(pref), Some(name)) => Some(format!("{pref}.{name}")),
        };

        out.push_str(render_class_docs(class_docs, &sub_prefix, 2, &renderer).trim_end());
        out.push('\n');
    }
    out
}

fn render_class_docs<R: Renderer>(
    class_docs: ClassDocumentation,
    prefix: &Option<String>,
    header_level: usize,
    renderer: &R,
) -> String {
    let mut out = String::new();
    out.push('\n');
    let fully_qualified_class_name = if let Some(p) = prefix {
        format!("{}.{}", p, &class_docs.name)
    } else {
        format!("{}", &class_docs.name)
    };

    out.push_str(&renderer.render_header(&fully_qualified_class_name, header_level));

    if let Some(docstring) = class_docs.docstring {
        let indent = detect_docstring_indent_prefix(&docstring);
        let docstring_ident_stripped = docstring
            .split("\n")
            .map(|s| s.strip_prefix(&indent).unwrap_or(s))
            .collect::<Vec<_>>()
            .join("\n");
        out.push('\n');
        out.push_str(docstring_ident_stripped.trim());
        out.push('\n');
    }
    let method_prefix = if let Some(p) = prefix {
        Some(format!("{}.{}", p, class_docs.name))
    } else {
        Some(class_docs.name.to_string())
    };
    for fn_docs in class_docs.methods {
        out.push('\n');
        out.push_str(
            render_function_docs(fn_docs, &method_prefix, header_level + 1, renderer).trim(),
        );
        out.push('\n');
    }
    out
}

fn render_function_docs<R: Renderer>(
    fn_docs: FunctionDocumentation,
    prefix: &Option<String>,
    header_level: usize,
    renderer: &R,
) -> String {
    let mut out = String::new();

    let fully_qualified_function_name = if let Some(p) = prefix {
        format!("{}.{}", p, &fn_docs.name)
    } else {
        fn_docs.name.to_string()
    };
    out.push_str(&renderer.render_header(&fully_qualified_function_name, header_level));

    out.push('\n');
    out.push_str(&fn_docs.name);
    out.push('(');
    out.push_str(&render_args(fn_docs.args));
    out.push(')');
    if let Some(return_annotation) = fn_docs.return_type {
        out.push_str(&format!(" -> {}", render_expr(return_annotation)));
    }
    out.push('\n');

    if let Some(docstring) = fn_docs.docstring {
        let indent = detect_docstring_indent_prefix(&docstring);
        let docstring_ident_stripped = docstring
            .split("\n")
            .map(|s| s.strip_prefix(&indent).unwrap_or(s))
            .collect::<Vec<_>>()
            .join("\n");
        out.push('\n');
        out.push_str(docstring_ident_stripped.trim());
    }
    out
}

/// Detects the common indentation prefix of a Python docstring.
/// Returns the leading whitespace (spaces/tabs) of the least-indented non-empty line after the first.
/// This handles both spaces and tabs without normalization.
fn detect_docstring_indent_prefix(docstring: &str) -> String {
    docstring
        .lines()
        .filter(|line| !line.trim().is_empty()) // skip lines that are fully empty or just whitespace
        .map(|line| {
            line.chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
        })
        .min_by_key(|prefix| prefix.len()) // get the shortest non-empty indent
        .unwrap_or_default()
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    use crate::{
        parsing::{module::extract_module_documentation, utils::parse_python_str},
        render::{
            formats::{md::MdRenderer, zola::ZolaRenderer},
            render_module, translate_filename,
        },
    };
    fn test_dirty_module_str() -> &'static str {
        r"'''This is a module that is used to test snakedown.'''

from typing import Any

__all__ = ['foo']

def foo(bar: int) -> Dict[str, Any]:
    '''this is a docstring for the foo function'''

    bar += 15
    bar << bar | 19
    return 0

class Greeter:
    '''
    this is a class docstring.

    '''

    class_var = 'whatever'

    def greet(self, name, *args, foo: str = 'bar', **kwargs) -> Callable[[], None]:
        '''





        Greet the world.

        Parameters
        ----------
        name: str
            just a parameter. it's actually used for anything

        Returns
        -------
        Callable[[], None]
            just a random closure to make the types interesting to render.




        '''
        print('Hello, world!')
        def inner():
            print('this is a closure!')
        inner()
        "
    }

    fn expected_module_docs_rendered() -> &'static str {
        r#"# snakedown.testing.test_module

This is a module that is used to test snakedown.

## snakedown.testing.test_module.foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## snakedown.testing.test_module.Greeter

this is a class docstring.

### snakedown.testing.test_module.Greeter.greet

greet(self, name, *args, foo: str = "bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }

    #[test]
    fn render_module_documentation() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(
            &parsed,
            Some(String::from("test_module")),
            Some(String::from("snakedown.testing")),
            false,
            false,
        );

        let rendered = render_module(mod_documentation, &MdRenderer::new());

        assert_eq!(rendered, expected_module_docs_rendered());

        Ok(())
    }
    fn expected_module_docs_no_prefix_no_name_rendered() -> &'static str {
        r#"
This is a module that is used to test snakedown.

## foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## Greeter

this is a class docstring.

### Greeter.greet

greet(self, name, *args, foo: str = "bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }

    fn expected_module_docs_only_prefix_rendered() -> &'static str {
        r#"# snakedown

This is a module that is used to test snakedown.

## snakedown.foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## snakedown.Greeter

this is a class docstring.

### snakedown.Greeter.greet

greet(self, name, *args, foo: str = "bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }

    #[test]
    fn render_module_documentation_no_prefix() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(&parsed, None, None, false, false);

        let rendered = render_module(mod_documentation, &MdRenderer::new());

        assert_eq!(rendered, expected_module_docs_no_prefix_no_name_rendered());

        Ok(())
    }
    #[test]
    fn render_module_documentation_only_prefix() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(
            &parsed,
            None,
            Some(String::from("snakedown")),
            false,
            false,
        );

        let rendered = render_module(mod_documentation, &MdRenderer::new());

        assert_eq!(rendered, expected_module_docs_only_prefix_rendered());

        Ok(())
    }

    fn expected_module_docs_only_name_rendered() -> &'static str {
        r#"# snakedown

This is a module that is used to test snakedown.

## snakedown.foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## snakedown.Greeter

this is a class docstring.

### snakedown.Greeter.greet

greet(self, name, *args, foo: str = "bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }
    fn expected_module_docs_zola_rendered() -> &'static str {
        r#"+++
title = "snakedown"
+++

This is a module that is used to test snakedown.

## snakedown.foo

foo(bar: int) -> Dict[str, Any]

this is a docstring for the foo function

## snakedown.Greeter

this is a class docstring.

### snakedown.Greeter.greet

greet(self, name, *args, foo: str = "bar", **kwargs) -> Callable[[], None]

Greet the world.

Parameters
----------
name: str
    just a parameter. it's actually used for anything

Returns
-------
Callable[[], None]
    just a random closure to make the types interesting to render.
"#
    }

    #[test]
    fn render_module_documentation_only_name() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(
            &parsed,
            Some("snakedown".to_string()),
            None,
            false,
            false,
        );

        let rendered = render_module(mod_documentation, &MdRenderer::new());

        assert_eq!(rendered, expected_module_docs_only_name_rendered());

        Ok(())
    }
    #[test]
    fn render_module_documentation_zola() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(
            &parsed,
            Some(String::from("snakedown")),
            None,
            false,
            false,
        );

        let rendered = render_module(mod_documentation, &ZolaRenderer::new(false));

        assert_eq!(rendered, expected_module_docs_zola_rendered());

        Ok(())
    }
    #[test]
    fn test_translate_filename_init() -> Result<()> {
        let input = PathBuf::from("foo/bar/__init__.py");
        let expected = PathBuf::from("foo/bar/_index.md");
        assert_eq!(translate_filename(&input), expected);
        Ok(())
    }
    #[test]
    fn test_translate_filename_module() -> Result<()> {
        let input = PathBuf::from("foo/bar/baz.py");
        let expected = PathBuf::from("foo/bar/baz.md");
        assert_eq!(translate_filename(&input), expected);
        Ok(())
    }
}
