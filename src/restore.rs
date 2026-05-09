use crate::models::{BackupError, Cli};

pub fn restore(cli: Cli) -> Result<(), BackupError> {
    if !cli.restore_path.is_file() {
        return Err(BackupError::TargetGameRestoreNotAFile {
            path: cli.restore_path.display().to_string(),
        });
    }

    Ok(())
}
