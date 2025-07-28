use std::path::PathBuf;

use url::Url;

#[derive(Debug)]
pub struct ObjectRef {
    pub import_path: String,
    pub object_name: String,
    pub link: Link,
}

#[derive(Debug)]
pub enum Link {
    Internal(PathBuf),
    External(Url),
}
