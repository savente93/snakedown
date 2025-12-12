use color_eyre::Result;
use rustpython_parser::{
    Mode,
    ast::{Constant, Expr, ExprConstant, Mod, Stmt, StmtExpr},
    parse,
};
use std::{fs::File, io::Read, path::Path};

pub fn parse_python_file(path: &Path) -> Result<Mod> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let program = parse_python_str(&file_content)?;
    Ok(program)
}

pub fn parse_python_str(content: &str) -> Result<Mod> {
    let parsed = parse(content, Mode::Module, "<embedded>");
    Ok(parsed?)
}

pub(crate) fn extract_docstring_from_body(body: &[Stmt]) -> Option<String> {
    match body.first() {
        Some(Stmt::Expr(StmtExpr { range: _, value })) => {
            if let Expr::Constant(ExprConstant {
                range: _,
                value: Constant::Str(s),
                kind: _,
            }) = &**value
            {
                Some(s.trim().to_string().clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {

    use crate::parsing::python::module::extract_module_documentation;

    use super::*;

    #[test]
    fn parse_empty_string() -> Result<()> {
        let program = parse_python_str("")?;
        let documentation = extract_module_documentation(&program, false, false);

        assert_eq!(documentation.docstring, None);
        assert_eq!(documentation.functions.len(), 0);
        assert_eq!(documentation.classes.len(), 0);

        Ok(())
    }
}
