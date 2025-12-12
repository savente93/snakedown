use crate::parsing::{
    ObjectDocumentation,
    python::{
        class::ClassDocumentation,
        function::FunctionDocumentation,
        module::{ModuleDocumentation, extract_module_documentation},
        utils::parse_python_file,
    },
};
use color_eyre::{Result, eyre::eyre};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Index {
    pub pkg_name: String,
    pub internal_object_store: HashMap<String, ObjectDocumentation>,
    pub skip_undoc: bool,
    pub skip_private: bool,
    pub pkg_root: PathBuf,
}

impl Index {
    pub fn new(pkg_root: PathBuf, skip_undoc: bool, skip_private: bool) -> Result<Self> {
        let pkg_name = pkg_root
            .file_stem()
            .and_then(|s| s.to_str())
            .map(String::from)
            .ok_or(eyre!("Error determining pkg_root name"))?;
        Ok(Self {
            pkg_name,
            internal_object_store: HashMap::new(),
            pkg_root,
            skip_undoc,
            skip_private,
        })
    }

    pub fn index_file(&mut self, path: PathBuf) -> Result<()> {
        tracing::info!("Indexing {}", &path.display());

        let parsed = parse_python_file(&path);

        let rel_module_file_path = path.clone().strip_prefix(&self.pkg_root)?.to_path_buf();
        let module_import_path: String = {
            let tmp_module_path =
                get_from_import_path(self.pkg_name.clone(), &rel_module_file_path)?;
            tmp_module_path
                .strip_suffix(".__init__")
                .unwrap_or(&tmp_module_path)
                .to_string()
        };

        match parsed {
            Ok(contents) => {
                let mod_docs =
                    extract_module_documentation(&contents, self.skip_private, self.skip_undoc);
                if should_include_module(&mod_docs, self.skip_undoc) {
                    self.internal_object_store.insert(
                        module_import_path.clone(),
                        ObjectDocumentation::Module(mod_docs.clone()),
                    );
                    for class_docs in &mod_docs.classes {
                        if should_include_class(class_docs, self.skip_private, self.skip_undoc) {
                            index_class(self, class_docs, module_import_path.clone())?;
                        }
                    }

                    for function_docs in mod_docs.functions {
                        if should_include_function(
                            &function_docs,
                            self.skip_private,
                            self.skip_undoc,
                        ) {
                            index_functions(self, &function_docs, module_import_path.clone())?;
                        }
                    }
                }

                Ok(())
            }
            Err(e) => {
                tracing::error!(
                    "The following error odducred while processing {}: {}",
                    &path.display(),
                    e
                );
                Err(e)
            }
        }
    }
}

pub fn should_include_class(
    class_docs: &ClassDocumentation,
    skip_private: bool,
    skip_undoc: bool,
) -> bool {
    (!skip_undoc || class_docs.docstring.is_some())
        && !(skip_private && class_docs.name.starts_with("_"))
}

pub fn should_include_function(
    func_docs: &FunctionDocumentation,
    skip_private: bool,
    skip_undoc: bool,
) -> bool {
    (!skip_undoc || func_docs.docstring.is_some())
        && !(skip_private && func_docs.name.starts_with("_"))
}

pub fn should_include_module(mod_docs: &ModuleDocumentation, skip_undoc: bool) -> bool {
    !skip_undoc || mod_docs.docstring.is_some()
}

pub fn index_functions(
    index: &mut Index,
    func_docs: &FunctionDocumentation,
    prefix: String,
) -> Result<()> {
    let full_prefix = format!("{}.{}", prefix, func_docs.name);
    tracing::debug!("Indexing {}", &full_prefix);

    // try_insert isn't stable yet
    #[allow(clippy::map_entry)]
    if index.internal_object_store.contains_key(&full_prefix) {
        Err(eyre!("tried to insert duplicate key: {}", &full_prefix))
    } else {
        index.internal_object_store.insert(
            full_prefix,
            ObjectDocumentation::Function(func_docs.clone()),
        );
        Ok(())
    }
}

pub fn index_class(
    index: &mut Index,
    class_docs: &ClassDocumentation,
    prefix: String,
) -> Result<()> {
    let full_prefix = format!("{}.{}", prefix, class_docs.name);
    tracing::debug!("Indexing {}", &full_prefix);

    if index.internal_object_store.contains_key(&full_prefix) {
        Err(eyre!("tried to insert duplicate key: {}", &full_prefix))
    } else {
        for meth_doc in &class_docs.methods {
            index_functions(index, meth_doc, full_prefix.clone())?;
        }
        index
            .internal_object_store
            .insert(full_prefix, ObjectDocumentation::Class(class_docs.clone()));
        Ok(())
    }
}

/// from import as in `from a.b.c import d`
///                         -----
pub fn get_from_import_path(pkg_name: String, relative_module_file_path: &Path) -> Result<String> {
    let mut import_components = vec![pkg_name];
    let components: Vec<String> = relative_module_file_path
        .with_extension("")
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .map(String::from)
        .collect::<Vec<String>>();

    import_components.extend(components);

    Ok(import_components.join("."))
}
