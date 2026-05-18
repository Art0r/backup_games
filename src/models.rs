use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub enum FuncType {
    Backup,
    Restore,
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Cli {
    pub fn backup_cli(from: String, to: Option<String>) -> Cli {
        let from_path = PathBuf::from(from);
        let home = env::var("HOME").expect("$HOME não existe");
        let backups_path = PathBuf::from(format!("{}/backps", home));
        let to_name = match to {
            Some(name) => name,
            None => "placeholder".to_string(),
        };

        if !backups_path.is_dir() {
            fs::create_dir(backups_path.clone())
                .expect("Ocorreu um erro ao criar a pasta ~/backups");
        }
        let to_path_file = PathBuf::from(format!("{}.zip", to_name));
        let to_path = backups_path.clone().join(to_path_file);

        Cli {
            from: from_path,
            to: to_path,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackupError {
    #[error("{path}")]
    TargetGameFolderPathNotFound { path: String },

    #[error("{path}")]
    TargetGameFolderPathNotDirectory { path: String },

    #[error("{path}")]
    TargetGameRestoreFileNotFound { path: String },

    #[error("{path}")]
    TargetGameRestoreNotAFile { path: String },

    #[error("{path}")]
    BackupPathError { path: String },

    #[error("{path}")]
    BackupFileError { path: String },

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

    #[error("{path}")]
    FileCreationError { path: String, error: String },
}
