use color_eyre::Result;
use std::{collections::HashMap, path::PathBuf};

use url::Url;

use crate::{
    indexing::object_ref::{Link, ObjectRef},
    parsing::sphinx::types::{ExternalSphinxRef, SphinxType},
};

#[derive(Debug)]
pub struct Index {
    index: HashMap<String, ObjectRef>,
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

impl Index {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
    pub fn insert_internal_ref(
        &mut self,
        import_path: String,
        object_name: String,
        path: PathBuf,
    ) -> Result<()> {
        self.index.insert(
            import_path.clone(),
            ObjectRef {
                import_path,
                object_name,
                link: Link::Internal(path),
            },
        );
        Ok(())
    }

    pub fn insert_external_ref(
        &mut self,
        import_path: String,
        object_name: String,
        url: Url,
    ) -> Result<()> {
        self.index.insert(
            import_path.clone(),
            ObjectRef {
                import_path,
                object_name,
                link: Link::External(url),
            },
        );

        Ok(())
    }

    pub fn get(&self, import_path: String) -> Option<&ObjectRef> {
        self.index.get(&import_path)
    }

    pub fn add_external_index(
        &mut self,
        base_url: Url,
        external_refs: Vec<ExternalSphinxRef>,
    ) -> Result<()> {
        // NOTE: things like labels and std have a bunch of weird stuff in them, that I doubt
        // people will want to link to, so for now I'm just doing the python stuff.
        // can reevaluate this if anyone asks for it.
        for external_ref in external_refs
            .into_iter()
            .filter(|e| matches!(e.sphinx_type, SphinxType::Python(_)))
        {
            let full_url = base_url.clone().join(&external_ref.location.to_string())?;

            self.index.insert(
                external_ref.name.clone(),
                ObjectRef {
                    import_path: external_ref.name,
                    object_name: external_ref.dispname,
                    link: Link::External(full_url),
                },
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use color_eyre::Result;
    use url::Url;

    use crate::{indexing::index::Index, parsing::sphinx::inv_file::parse_objects_inv_file};

    #[test]
    fn index_numpy_objects_inv() -> Result<()> {
        let base_url = Url::parse("https://numpy.org/doc/stable/")?;
        let numpy_refs = parse_objects_inv_file(&PathBuf::from("tests/sphinx_objects/numpy.inv"))?;

        let mut index = Index::new();

        index.add_external_index(base_url, numpy_refs)?;

        Ok(())
    }
}
