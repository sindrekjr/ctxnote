use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Storage {
    Db { conn: String },
    Fs { dir: PathBuf },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataConfig {
    #[serde(default = "default_storage")]
    pub storage: Storage,
}

fn default_storage() -> Storage {
    Storage::Fs { dir: default_dir() }
}

fn default_dir() -> PathBuf {
    ProjectDirs::from("", "", "ctxnote").unwrap().data_dir().to_path_buf()
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            storage: default_storage(),
        }
    }
}
