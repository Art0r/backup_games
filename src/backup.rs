use std::fs::File;
use walkdir::WalkDir;

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::models::{BackupError, Cli};

pub fn backup(cli: Option<Cli>) -> Result<(), BackupError> {
    let cli = match cli {
        Some(cli) => cli,
        None => {
            let from = std::env::args()
                .nth(2)
                .expect("Folder to backup must be specified");
            let to = std::env::args().nth(3);

            Cli::backup_cli(from, to)
        }
    };

    if !cli.from.exists() {
        return Err(BackupError::TargetGameFolderPathNotFound {
            path: cli.from.display().to_string(),
        });
    }

    if !cli.from.is_dir() {
        return Err(BackupError::TargetGameFolderPathNotDirectory {
            path: cli.from.display().to_string(),
        });
    }

    let file = File::create(cli.to.clone()).map_err(|e| BackupError::FileCreationError {
        path: cli.to.display().to_string(),
        error: e.to_string(),
    })?;

    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(&cli.from);

    for entry in walkdir.into_iter() {
        let entry_result = entry.map_err(|e| BackupError::WalkDirError {
            path: cli.from.display().to_string(),
            source: e,
        })?;

        let path = entry_result.path();
        let path_striped =
            path.strip_prefix(cli.from.clone())
                .map_err(|_| BackupError::StripPrefixError {
                    path: path.display().to_string(),
                    prefix: cli.from.display().to_string(),
                })?;

        let path_as_string = path_striped.display().to_string();

        if path.is_file() {
            zip.start_file(path_as_string, options)
                .map_err(|_| BackupError::ZipPathError {
                    path: path.display().to_string(),
                    prefix: cli.from.display().to_string(),
                })?;

            let mut f = File::open(path).map_err(|_| BackupError::ZipPathError {
                path: path.display().to_string(),
                prefix: cli.from.display().to_string(),
            })?;

            std::io::copy(&mut f, &mut zip).map_err(|_| BackupError::BackupPathError {
                path: path.display().to_string(),
            })?;
        } else if !path_striped.as_os_str().is_empty() {
            zip.add_directory(path_as_string, options)
                .map_err(|_| BackupError::ZipPathError {
                    path: path.display().to_string(),
                    prefix: cli.from.display().to_string(),
                })?;
        }
    }
    zip.finish().map_err(|_| BackupError::ZipPathError {
        path: cli.from.display().to_string(),
        prefix: cli.from.display().to_string(),
    })?;

    Ok(())
}
