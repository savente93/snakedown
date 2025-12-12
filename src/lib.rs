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
use crate::indexing::index::Index;
use crate::render::formats::Renderer;
pub use crate::render::render_module;
use crate::render::render_object;

use color_eyre::Result;

pub fn render_docs<R: Renderer>(
    pkg_path: &Path,
    out_path: &Path,
    skip_private: bool,
    skip_undoc: bool,
    exclude: Vec<PathBuf>,
    renderer: &R,
) -> Result<Vec<PathBuf>> {
    // let root_pkg_name = get_module_name(pkg_path)?;
    let absolute_pkg_path = pkg_path.canonicalize()?;
    let errored = vec![];

    tracing::info!("indexing package at {}", &absolute_pkg_path.display());
    let mut index = Index::new(absolute_pkg_path.clone(), skip_undoc, skip_private)?;

    crawl_package(
        &mut index,
        &absolute_pkg_path,
        skip_private,
        exclude.clone(),
    )?;

    create_dir_all(out_path)?;

    for (key, object) in index.internal_object_store.iter() {
        let file_path = out_path.join(key).with_added_extension("md");
        let rendered = render_object(object, key.clone(), renderer);
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
    pub fn assert_dir_trees_equal<P: AsRef<Path>>(dir1: P, dir2: P) {
        match compare_dirs(dir1.as_ref(), dir2.as_ref()) {
            Ok(_) => (),
            Err(e) => panic!("Directory trees differ:\n{e}"),
        }
    }

    #[allow(clippy::unwrap_used)]
    fn compare_dirs(dir1: &Path, dir2: &Path) -> Result<()> {
        let entries1 = collect_files(dir1)?;
        let entries2 = collect_files(dir2)?;

        let mut errors = Vec::new();

        // Get all unique relative paths from both directories
        let paths1: HashSet<_> = entries1.keys().collect();
        let paths2: HashSet<_> = entries2.keys().collect();

        let only_in_1 = paths1.difference(&paths2);
        let only_in_2 = paths2.difference(&paths1);
        let mut in_both: Vec<_> = paths1.intersection(&paths2).collect();

        in_both.sort();

        for path in only_in_1 {
            errors.push(format!("Only in {dir1:?}: {path:?}"));
        }

        for path in only_in_2 {
            errors.push(format!("Only in {dir2:?}: {path:?}"));
        }

        for path in in_both {
            let full1 = entries1.get(*path).unwrap();
            let full2 = entries2.get(*path).unwrap();

            let meta1 = full1.metadata().wrap_err("reading metadata 1")?;
            let meta2 = full2.metadata().wrap_err("reading metadata 2")?;

            match (meta1.is_file(), meta2.is_file()) {
                (true, true) => {
                    if let Err(e) = compare_files(full1, full2) {
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
    fn compare_files(path1: &Path, path2: &Path) -> io::Result<()> {
        let mut file1 = fs::File::open(path1)?;
        let mut file2 = fs::File::open(path2)?;

        let mut buf1 = String::new();
        let mut buf2 = String::new();

        file1.read_to_string(&mut buf1)?;
        file2.read_to_string(&mut buf2)?;

        buf1 = buf1.replace(" ", "");
        buf1 = buf1.replace("\n", "");
        buf1 = buf1.replace("\t", "");
        buf2 = buf2.replace(" ", "");
        buf2 = buf2.replace("\n", "");
        buf2 = buf2.replace("\t", "");

        assert_eq!(buf1, buf2, "{} is different", path1.display());

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

        assert_dir_trees_equal(temp_dir.path(), &expected_result_dir);

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

        assert_dir_trees_equal(temp_dir.path(), &expected_result_dir);

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
