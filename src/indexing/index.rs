use crate::{
    parsing::{
        ObjectDocumentation,
        python::{
            class::ClassDocumentation,
            function::FunctionDocumentation,
            module::{ModuleDocumentation, extract_module_documentation},
            utils::parse_python_file,
        },
    },
    render::formats::Renderer,
};
use color_eyre::{Report, Result, eyre::eyre};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use url::Url;

// TODO: I think the winning strategy here is to start with a `RawIndex` that is more or less the
// current index struct, which refers to all the original python stuff on disk with all the type
// info etc.
// then we do something like `let serializable_index = index.process()?;` with something like
// `RawIndex::process(self) -> SerializableIndex` which is a new object with everything
// preprocessed, incl stripped prefixes, all references validated and expanded etc.
// then later we can just write everything to disk separately. That's a nice separation of concerns
// see also: https://github.com/savente93/snakedown/issues/57

#[derive(Debug)]
pub struct RawIndex {
    pub pkg_name: String,
    pub internal_object_store: HashMap<String, ObjectDocumentation>,
    pub external_object_store: HashMap<String, Url>,
    pub skip_undoc: bool,
    pub skip_private: bool,
    pub pkg_root: PathBuf,
}

impl RawIndex {
    pub fn new(pkg_root: PathBuf, skip_undoc: bool, skip_private: bool) -> Result<Self> {
        let pkg_name = pkg_root
            .file_stem()
            .and_then(|s| s.to_str())
            .map(String::from)
            .ok_or(eyre!("Error determining pkg_root name"))?;
        Ok(Self {
            pkg_name,
            internal_object_store: HashMap::new(),
            external_object_store: HashMap::new(),
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

    pub fn validate_references(&self) -> Result<(), Vec<Report>> {
        let mut errors: Vec<_> = Vec::new();
        for (key, obj) in self.internal_object_store.iter() {
            if let Some((_, used_references)) = obj.extract_used_references() {
                for used_ref in used_references {
                    if !self
                        .internal_object_store
                        .contains_key(&used_ref.fully_qualified_name)
                    {
                        errors.push(eyre!(
                            "unknown reference: {}, in object {}",
                            used_ref.fully_qualified_name,
                            key
                        ));
                    }
                }
            };
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    //TODO: This is not an efficient way to do this, but for the test cases it works,
    //at some point we should find a more high performance solution.
    // see: https://github.com/savente93/snakedown/issues/55
    pub fn pre_process<R: Renderer>(&mut self, render: R) -> Result<()> {
        for (_key, object) in self.internal_object_store.iter_mut() {
            if let Some((mut object_docstring, used_references)) = object.extract_used_references()
            {
                for used_ref in used_references {
                    let display_text = used_ref
                        .clone()
                        .display_text
                        .or_else(|| Some(used_ref.fully_qualified_name.clone()));
                    let expanded_ref = render
                        .render_reference(display_text, used_ref.fully_qualified_name.clone())?;
                    object_docstring =
                        object_docstring.replace(&used_ref.original(), &expanded_ref);
                }
                object.replace_docstring(Some(object_docstring));
            }
        }

        Ok(())
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
    index: &mut RawIndex,
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
    index: &mut RawIndex,
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
