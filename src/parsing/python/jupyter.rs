use color_eyre::{Result, eyre::eyre};
use nbformat::{Notebook, parse_notebook, upgrade_legacy_notebook, v4::Cell};
use std::{fs, path::Path};

pub struct ParsedNotebook {
    name: String,
    cells: Vec<Cell>,
}

pub fn parse_notebook_file(path: &Path) -> Result<Vec<Cell>> {
    // Read the notebook file
    let notebook_json = fs::read_to_string(path)?;

    // Parse the notebook
    let notebook = match parse_notebook(&notebook_json)? {
        Notebook::V4(notebook) => notebook,
        Notebook::Legacy(notebook) => {
            upgrade_legacy_notebook(notebook).map_err(|err| eyre!(err))?
        }
    };

    let metadata = notebook.metadata;

    let language_name = match (metadata.kernelspec, metadata.language_info) {
        (None, None) => None,
        (_, Some(li)) => Some(li.name),
        (Some(ks), None) => ks.language,
    };

    if language_name != Some("pyhton".to_string()) {
        return Err(eyre!(
            "Currently languages other than python are not supported in snakedown"
        ));
    };

    Ok(notebook.cells)
}

#[cfg(test)]
mod test {

    use color_eyre::Result;
    use std::path::PathBuf;

    use crate::parsing::python::jupyter::parse_notebook_file;

    fn test_notebook_path() -> PathBuf {
        PathBuf::from("tests/test_notebooks/pandas_example.ipynb")
    }

    #[test]
    fn test_can_parse_pandas_example() -> Result<()> {
        let path = test_notebook_path();

        let notebook = parse_notebook_file(&path)?;

        dbg!(notebook);

        assert!(false);

        Ok(())
    }
}
