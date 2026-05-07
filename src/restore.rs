use std::io::Error;

use crate::models::Cli;

pub fn restore(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if !cli.restore_path.is_file() {
        panic!("Restore is not a file")
    }

    Ok(())
}
