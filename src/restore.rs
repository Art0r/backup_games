use std::io::Error;

use crate::models::{BackupError, Cli};

pub fn restore(cli: Cli) -> Result<(), BackupError> {
    if !cli.restore_path.is_file() {
        panic!("Restore is not a file")
    }

    Ok(())
}
