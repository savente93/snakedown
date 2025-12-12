use crate::parsing::ObjectDocumentation;
use std::{collections::HashMap, path::PathBuf};

pub struct Index {
    pub internal_object_store: HashMap<String, ObjectDocumentation>,
    pub skip_undoc: bool,
    pub skip_private: bool,
    pub pkg_root: PathBuf,
}

impl Index {
    pub fn new(pkg_root: PathBuf, skip_undoc: bool, skip_private: bool) -> Self {
        Self {
            internal_object_store: HashMap::new(),
            pkg_root,
            skip_undoc,
            skip_private,
        }
    }
}
