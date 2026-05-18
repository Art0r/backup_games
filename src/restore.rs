use std::{
    fs::File,
    io::{BufReader, Read},
};

use zip::{ZipArchive, read::ZipFile};

use crate::models::{BackupError, Cli};

pub fn restore() -> Result<(), BackupError> {
    // if !cli.restore_path.is_file() {
    //     return Err(BackupError::TargetGameRestoreNotAFile {
    //         path: cli.restore_path.display().to_string(),
    //     });
    // }
    //
    // if !cli.restore_path.exists() {
    //     return Err(BackupError::TargetGameRestoreFileNotFound {
    //         path: cli.restore_path.display().to_string(),
    //     });
    // }
    //
    // if !cli.backup_path.exists() & !cli.backup_path.is_file() {
    //     return Err(BackupError::TargetGameFolderPathNotFound {
    //         path: cli.restore_path.display().to_string(),
    //     });
    // }
    //
    // let restore_file =
    //     File::open(cli.restore_path.clone()).map_err(|_| BackupError::BackupFileError {
    //         path: cli.restore_path.display().to_string(),
    //     })?;
    //
    // let restore_file_zip = ZipArchive::new(restore_file);
    //
    // for file in restore_file_zip.iter().by_ref() {
    //     for name in file.file_names() {
    //         eprintln!("{}", name.to_string());
    //     }
    // }
    //
    Ok(())
}
