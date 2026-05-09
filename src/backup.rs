use std::fs::File;
use walkdir::WalkDir;

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::models::{BackupError, Cli};

pub fn backup(cli: Cli) -> Result<(), BackupError> {
    if !cli.backup_path.exists() {
        return Err(BackupError::BackupPathNotFound {
            path: cli.backup_path.display().to_string(),
        });
    }

    if !cli.backup_path.is_dir() {
        return Err(BackupError::BackupPathNotDirectory {
            path: cli.backup_path.display().to_string(),
        });
    }

    let file = File::create(cli.restore_path.clone()).map_err(|_| BackupError::ZipPathError {
        path: cli.backup_path.display().to_string(),
        prefix: cli.backup_path.display().to_string(),
    })?;

    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(&cli.backup_path);

    for entry in walkdir.into_iter() {
        let entry_result = entry.map_err(|e| BackupError::WalkDirError {
            path: cli.backup_path.display().to_string(),
            source: e,
        })?;

        let path = entry_result.path();
        let path_striped = path.strip_prefix(cli.backup_path.clone()).map_err(|_| {
            BackupError::StripPrefixError {
                path: path.display().to_string(),
                prefix: cli.backup_path.display().to_string(),
            }
        })?;

        let path_as_string = path_striped.display().to_string();

        if path.is_file() {
            zip.start_file(path_as_string, options)
                .map_err(|_| BackupError::ZipPathError {
                    path: path.display().to_string(),
                    prefix: cli.backup_path.display().to_string(),
                })?;

            let mut f = File::open(path).map_err(|_| BackupError::ZipPathError {
                path: path.display().to_string(),
                prefix: cli.backup_path.display().to_string(),
            })?;

            std::io::copy(&mut f, &mut zip).map_err(|_| BackupError::BackupPathError {
                path: path.display().to_string(),
            })?;
        } else if !path_striped.as_os_str().is_empty() {
            zip.add_directory(path_as_string, options)
                .map_err(|_| BackupError::ZipPathError {
                    path: path.display().to_string(),
                    prefix: cli.backup_path.display().to_string(),
                })?;
        }
    }
    zip.finish().map_err(|_| BackupError::ZipPathError {
        path: cli.backup_path.display().to_string(),
        prefix: cli.backup_path.display().to_string(),
    })?;

    Ok(())
}
