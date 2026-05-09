use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FuncType {
    Backup,
    Restore,
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub func: FuncType,
    pub backup_path: PathBuf,
    pub restore_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum BackupError {
    #[error("Backup path does not exist: {path}")]
    BackupPathNotFound { path: String },

    #[error("Backup path is not a directory: {path}")]
    BackupPathNotDirectory { path: String },

    #[error("Failed to read directory '{path}': {source}")]
    WalkDirError {
        path: String,
        #[source]
        source: walkdir::Error,
    },

    #[error("Failed to strip prefix '{prefix}' from path '{path}'")]
    StripPrefixError { path: String, prefix: String },

    #[error("Zip path error prefix '{prefix}' from path '{path}'")]
    ZipPathError { path: String, prefix: String },
}
