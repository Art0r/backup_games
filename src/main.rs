mod backup;
mod models;
mod restore;

use crate::backup::backup;
use crate::models::{Cli, FuncType};
use crate::restore::restore;
use std::path::PathBuf;

fn main() {
    let func_arg = std::env::args().nth(1).expect("No function specified");
    let func = match func_arg.to_lowercase().as_str() {
        "backup" => FuncType::Backup,
        "restore" => FuncType::Restore,
        _ => panic!("Invalid function: {}. Use 'backup' or 'restore'", func_arg),
    };

    let backup_arg = std::env::args().nth(2).expect("Backup path must be set");
    let backup_path = PathBuf::from(backup_arg);

    let restore_arg = std::env::args().nth(3).expect("Restore path must be set");
    let restore_path = PathBuf::from(restore_arg);

    let cli = Cli {
        func,
        backup_path,
        restore_path,
    };

    match cli.func {
        FuncType::Backup => backup(cli.clone()),
        FuncType::Restore => restore(cli.clone()),
    }
    .unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    });
}
