pub mod config;
pub mod fs;
pub mod indexing;
pub mod parsing;
pub mod render;

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::fs::crawl_package;
pub use crate::fs::{get_module_name, get_package_modules, walk_package};
use crate::indexing::index::RawIndex;
use crate::render::formats::Renderer;
pub use crate::render::render_module;
use crate::render::render_object;

use color_eyre::Result;
use color_eyre::eyre::eyre;
use tera::Context;

pub fn render_docs<R: Renderer>(
    pkg_path: &Path,
    out_path: &Path,
    skip_private: bool,
    skip_undoc: bool,
    exclude: Vec<PathBuf>,
    renderer: &R,
) -> Result<Vec<PathBuf>> {
    let absolute_pkg_path = pkg_path.canonicalize()?;
    let errored = vec![];

    let mut ctx = Context::new();
    let sd_version = env!("CARGO_PKG_VERSION_MAJOR");
    ctx.insert("SNAKEDOWN_VERSION", &sd_version);

    tracing::info!("indexing package at {}", &absolute_pkg_path.display());
    let mut index = RawIndex::new(absolute_pkg_path.clone(), skip_undoc, skip_private)?;

    crawl_package(
        &mut index,
        &absolute_pkg_path,
        skip_private,
        exclude.clone(),
    )?;

    match index.validate_references() {
        Ok(_) => Ok(()),
        Err(errors) => Err(eyre!(
            "Found {} invalid references(s):\n{:?}",
            errors.len(),
            errors
        )),
    }?;

    index.pre_process(renderer)?;

    create_dir_all(out_path)?;

    for (key, object) in index.internal_object_store.iter() {
        let file_path = out_path.join(key).with_added_extension("md");
        let rendered = render_object(object, key.clone(), renderer, &ctx)?;
        let rendered_trimmed = rendered.trim_start();
        let mut file = File::create(file_path)?;
        file.write_all(rendered_trimmed.as_bytes())?;
    }

    Ok(errored)
}

#[cfg(test)]
mod test {

    use std::path::{Path, PathBuf};

    use crate::render::formats::md::MdRenderer;

    use crate::render_docs;

    use pretty_assertions::assert_eq;
    use std::collections::HashSet;
    use std::fs;
    use std::io::{self, Read};

    use color_eyre::eyre::{Result, WrapErr, eyre};
    use walkdir::WalkDir;

    /// Asserts that two directory trees are identical in structure and content.
    /// Reports all differences including missing files and content mismatches.
    pub fn assert_dir_trees_equal<P: AsRef<Path>>(expected: P, actual: P) {
        match compare_dirs(expected.as_ref(), actual.as_ref()) {
            Ok(_) => (),
            Err(e) => panic!("Directory trees differ:\n{e}"),
        }
    }

    #[allow(clippy::unwrap_used)]
    fn compare_dirs(expected: &Path, actual: &Path) -> Result<()> {
        let entries_expected = collect_files(expected)?;
        let entries_actual = collect_files(actual)?;

        let mut errors = Vec::new();

        // Get all unique relative paths from both directories
        let paths_expected: HashSet<_> = entries_expected.keys().collect();
        let paths_actual: HashSet<_> = entries_actual.keys().collect();

        let only_in_expected = paths_expected.difference(&paths_actual);
        let only_in_actual = paths_actual.difference(&paths_expected);
        let mut in_both: Vec<_> = paths_expected.intersection(&paths_actual).collect();

        in_both.sort();

        for path in only_in_expected {
            errors.push(format!("Only in {expected:?}(expected): {path:?}"));
        }

        for path in only_in_actual {
            errors.push(format!("Only in {actual:?}(actual): {path:?}"));
        }

        for path in in_both {
            let full_expected = entries_expected.get(*path).unwrap();
            let full_actual = entries_actual.get(*path).unwrap();

            let meta_expected = full_expected.metadata().wrap_err("reading metadata 1")?;
            let meta_actual = full_actual.metadata().wrap_err("reading metadata 2")?;

            match (meta_expected.is_file(), meta_actual.is_file()) {
                (true, true) => {
                    if let Err(e) = compare_files(full_expected, full_actual) {
                        errors.push(format!("Content differs at {path:?}: {e}"));
                    }
                }
                (false, false) => {} // Both are directories, skip
                _ => {
                    errors.push(format!("Type mismatch at {path:?}: file vs directory"));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(eyre!(
                "Found {} difference(s):\n{}",
                errors.len(),
                errors.join("\n")
            ))
        }
    }

    /// Recursively collects all files and directories with paths relative to `base`.
    fn collect_files(base: &Path) -> Result<std::collections::HashMap<PathBuf, PathBuf>> {
        let mut map = std::collections::HashMap::new();
        for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            let rel = path.strip_prefix(base)?;
            map.insert(rel.to_path_buf(), path.to_path_buf());
        }
        Ok(map)
    }

    /// Compares the content of two files.
    fn compare_files(expected: &Path, actual: &Path) -> io::Result<()> {
        let mut file_expected = fs::File::open(expected)?;
        let mut file_actual = fs::File::open(actual)?;

        let mut buf_expected = String::new();
        let mut buf_actual = String::new();

        file_expected.read_to_string(&mut buf_expected)?;
        file_actual.read_to_string(&mut buf_actual)?;

        assert_eq!(
            buf_expected,
            buf_actual,
            "{} is different",
            expected.display()
        );

        Ok(())
    }

    #[test]
    fn render_test_pkg_docs_full() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let test_pkg_dir = PathBuf::from("tests/test_pkg");
        let expected_result_dir = PathBuf::from("tests/rendered_full");

        render_docs(
            &test_pkg_dir,
            temp_dir.path(),
            false,
            false,
            vec![
                PathBuf::from("test_pkg/excluded_file.py"),
                PathBuf::from("test_pkg/excluded_module"),
            ],
            &MdRenderer::new(),
        )?;

        assert_dir_trees_equal(expected_result_dir.as_path(), temp_dir.path());

        Ok(())
    }
    #[test]
    fn render_test_pkg_docs_no_private_no_undoc() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let test_pkg_dir = PathBuf::from("tests/test_pkg");
        let expected_result_dir = PathBuf::from("tests/rendered_no_private");

        render_docs(
            &test_pkg_dir,
            temp_dir.path(),
            true,
            true,
            vec![
                PathBuf::from("test_pkg/excluded_file.py"),
                PathBuf::from("test_pkg/excluded_module"),
            ],
            &MdRenderer::new(),
        )?;

        assert_dir_trees_equal(expected_result_dir.as_path(), temp_dir.path());

        Ok(())
    }
    #[test]
    fn render_test_pkg_docs_exit_on_err() -> Result<()> {
        let temp_dir = assert_fs::TempDir::new()?;
        let test_pkg_dir = PathBuf::from("tests/test_pkg");

        render_docs(
            &test_pkg_dir,
            temp_dir.path(),
            false,
            false,
            vec![],
            &MdRenderer::new(),
        )?;

        Ok(())
    }
}
