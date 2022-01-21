use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub enum Storage {
    //TODO: Db,
    Fs,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataConfig {
    pub storage: Storage,
    pub dir: PathBuf,
}

impl Default for DataConfig {
    fn default() -> Self {
        let dirs = ProjectDirs::from("", "", "ctxnote").unwrap();
        Self {
            storage: Storage::Fs,
            dir: dirs.data_dir().to_path_buf(),
        }
    }
}
