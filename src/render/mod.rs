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
        ObjectDocumentation,
        python::{
            class::ClassDocumentation, function::FunctionDocumentation, module::ModuleDocumentation,
        },
    },
    render::formats::Renderer,
};

#[derive(Clone, Copy, Debug, Display, ValueEnum, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase"))]
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

pub fn fully_qualified_object_name(object: &ObjectDocumentation, prefix: Option<String>) -> String {
    match (object, prefix) {
        (ObjectDocumentation::Module(_), None) => String::new(),
        (ObjectDocumentation::Module(_), Some(p)) => p,
        (ObjectDocumentation::Class(class_documentation), None) => {
            class_documentation.name.to_string().trim().to_string()
        }
        (ObjectDocumentation::Class(class_documentation), Some(p)) => {
            format!("{}.{}", p, class_documentation.name.clone().trim())
        }
        (ObjectDocumentation::Function(function_documentation), None) => {
            function_documentation.name.to_string().trim().to_string()
        }
        (ObjectDocumentation::Function(function_documentation), Some(p)) => {
            format!("{}.{}", p, function_documentation.name.to_string().trim())
        }
    }
}

pub fn render_object<R: Renderer>(
    object: &ObjectDocumentation,
    fully_qualified_name: String,
    renderer: &R,
) -> String {
    match object {
        ObjectDocumentation::Class(class_documentation) => {
            render_class_docs(class_documentation, &fully_qualified_name, renderer)
        }
        ObjectDocumentation::Module(module_documentation) => {
            render_module(module_documentation, fully_qualified_name, renderer)
        }
        ObjectDocumentation::Function(function_documentation) => {
            render_function_docs(function_documentation, &fully_qualified_name, renderer)
        }
    }
}

pub fn render_module<R: Renderer>(
    mod_doc: &ModuleDocumentation,
    fully_qualified_name: String,
    renderer: &R,
) -> String {
    let mut out = String::new();

    let front_matter_str = renderer.render_front_matter(Some(&fully_qualified_name));
    if !front_matter_str.is_empty() {
        out.push_str(&front_matter_str);
        out.push('\n');
        out.push('\n');
    }

    if let Some(docstring) = &mod_doc.docstring {
        out.push_str(docstring);
        out.push('\n');
    }

    out
}

fn render_class_docs<R: Renderer>(
    class_docs: &ClassDocumentation,
    fully_qualified_name: &str,
    renderer: &R,
) -> String {
    let mut out = String::new();

    out.push_str(&renderer.render_front_matter(Some(fully_qualified_name)));
    out.push('\n');
    out.push('\n');

    if let Some(docstring) = &class_docs.docstring {
        out.push_str(docstring);
        out.push('\n');
    }

    out
}

fn render_function_docs<R: Renderer>(
    fn_docs: &FunctionDocumentation,
    fully_qualified_name: &str,
    renderer: &R,
) -> String {
    let mut out = String::new();

    out.push_str(&renderer.render_front_matter(Some(fully_qualified_name)));

    out.push('\n');
    out.push('\n');
    out.push_str(&fn_docs.name);
    out.push('(');
    out.push_str(&render_args(fn_docs.args.clone()));
    out.push(')');
    if let Some(return_annotation) = fn_docs.return_type.clone() {
        out.push_str(&format!(" -> {}", render_expr(return_annotation)));
    }

    if let Some(docstring) = fn_docs.docstring.clone() {
        out.push('\n');
        out.push('\n');
        out.push_str(docstring.trim());
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    use crate::{
        parsing::{python::module::extract_module_documentation, python::utils::parse_python_str},
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
"#
    }

    #[test]
    fn render_module_documentation() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(&parsed, false, false);

        let rendered = render_module(
            &mod_documentation,
            String::from("snakedown.testing.test_module"),
            &MdRenderer::new(),
        );

        assert_eq!(rendered, expected_module_docs_rendered());

        Ok(())
    }

    fn expected_module_docs_zola_rendered() -> &'static str {
        r#"+++
title = "snakedown"
+++

This is a module that is used to test snakedown.
"#
    }

    #[test]
    fn render_module_documentation_zola() -> Result<()> {
        let parsed = parse_python_str(test_dirty_module_str())?;
        let mod_documentation = extract_module_documentation(&parsed, false, false);

        let rendered = render_module(
            &mod_documentation,
            String::from("snakedown"),
            &ZolaRenderer::new(false),
        );

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
