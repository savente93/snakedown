pub mod fs;
pub mod parsing;
pub mod render;

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

pub use crate::fs::{get_module_name, get_package_modules, walk_package};
use crate::render::formats::Renderer;
pub use crate::render::render_module;

use color_eyre::Result;
use fs::get_python_prefix;
use parsing::module::extract_module_documentation;
use parsing::utils::parse_python_file;
use render::translate_filename;

pub fn render_docs<R: Renderer>(
    pkg_path: &Path,
    out_path: &Path,
    skip_private: bool,
    skip_undoc: bool,
    exclude: Vec<PathBuf>,
    renderer: &R,
) -> Result<Vec<PathBuf>> {
    let root = pkg_path;
    let root_pkg_path = get_module_name(pkg_path)?;
    let mut errored = vec![];

    tracing::info!("indexing package at {}", &pkg_path.display());
    let pkg_index = walk_package(pkg_path, skip_private, exclude)?;

    tracing::info!("Creating directories");

    for sub_pkg in pkg_index.package_paths {
        tracing::debug!("Creating directory: {}", &sub_pkg.display());
        let rel_write_path = sub_pkg.strip_prefix(root)?;
        let full_write_path = out_path.join(rel_write_path);
        create_dir_all(&full_write_path)?;
    }
    tracing::info!("done creating directories");

    for sub_module in pkg_index.module_paths {
        tracing::info!("creating documentation for {}", &sub_module.display());
        let rel_write_path = sub_module.strip_prefix(root)?;
        let rel_python_path = Path::new(&root_pkg_path).join(rel_write_path);
        let full_write_path = out_path.join(rel_write_path);
        let prefix = get_python_prefix(&rel_python_path)?;
        let parsed = parse_python_file(&sub_module);
        match parsed {
            Ok(contents) => {
                tracing::debug!("correctly parsed file {}", &sub_module.display());
                tracing::debug!("extracting documentation...");
                let module_name = get_module_name(&sub_module).ok();
                let documentation = {
                    // extra scope is so docs doesn't have to be mutable for the
                    // whole rest of the function
                    let mut tmp_docs = extract_module_documentation(
                        &contents,
                        module_name,
                        prefix,
                        skip_private,
                        skip_undoc,
                    );
                    if sub_module.ends_with("__init__.py") {
                        if let Some(dir) = sub_module.parent() {
                            tmp_docs.with_sub_modules(
                                pkg_index.sub_module_index.get(&dir.to_path_buf()),
                            );
                        }
                    }
                    tmp_docs
                };
                tracing::debug!("rendering documentation...");
                let rendered = render_module(documentation, &renderer);
                let new_write_path = translate_filename(&full_write_path);
                tracing::debug!(
                    "writing rendered documentation too {}",
                    &new_write_path.display()
                );
                let mut file = File::create(new_write_path)?;
                file.write_all(rendered.as_bytes())?;
            }
            Err(e) => {
                tracing::error!(
                    "The following error occurred while processing {}: {}",
                    &sub_module.display(),
                    e
                );
                errored.push(sub_module);
            }
        }
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
            Err(e) => panic!("Directory trees differ:\n{}", e),
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
        let in_both = paths1.intersection(&paths2);

        for path in only_in_1 {
            errors.push(format!("Only in {:?}: {:?}", dir1, path));
        }

        for path in only_in_2 {
            errors.push(format!("Only in {:?}: {:?}", dir2, path));
        }

        for path in in_both {
            let full1 = entries1.get(*path).unwrap();
            let full2 = entries2.get(*path).unwrap();

            let meta1 = full1.metadata().wrap_err("reading metadata 1")?;
            let meta2 = full2.metadata().wrap_err("reading metadata 2")?;

            match (meta1.is_file(), meta2.is_file()) {
                (true, true) => {
                    if let Err(e) = compare_files(full1, full2) {
                        errors.push(format!("Content differs at {:?}: {}", path, e));
                    }
                }
                (false, false) => {} // Both are directories, skip
                _ => {
                    errors.push(format!("Type mismatch at {:?}: file vs directory", path));
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

        assert_eq!(buf1, buf2,);

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
