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
    #[serde(default = "default_storage")]
    pub storage: Storage,
    #[serde(default = "default_dir")]
    pub dir: PathBuf,
}

fn default_storage() -> Storage {
    Storage::Fs
}

fn default_dir() -> PathBuf {
    ProjectDirs::from("", "", "ctxnote").unwrap().data_dir().to_path_buf()
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            storage: default_storage(),
            dir: default_dir(),
        }
    }
}
