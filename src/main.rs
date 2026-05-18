mod backup;
mod models;
mod restore;

use crate::backup::backup;
use crate::models::{Cli, FuncType};
use crate::restore::restore;
use std::env;
use std::path::PathBuf;

pub enum PredefinedFunc {
    CalibreBackup,
}

fn main() {
    let func_arg = std::env::args().nth(1).expect("No function specified");
    let func = match func_arg.to_lowercase().as_str() {
        "backup" => FuncType::Backup,
        "restore" => FuncType::Restore,
        _ => panic!("Invalid function: {}. Use 'backup' or 'restore'", func_arg),
    };

    let predefined_func_arg = std::env::args()
        .nth(2)
        .expect("Needs at least a second arg");
    let predefined_func: Option<PredefinedFunc> = match predefined_func_arg.to_lowercase().as_str()
    {
        "calibre_backup" => Some(PredefinedFunc::CalibreBackup),
        _ => None,
    };

    if predefined_func.is_none() {
        match func {
            FuncType::Backup => backup(None),
            FuncType::Restore => restore(),
        }
        .unwrap_or_else(|e| {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        });
    } else {
        let home = env::var("HOME").expect("$HOME não existe");

        match predefined_func.expect("PredefinedFunc error") {
            PredefinedFunc::CalibreBackup => {
                let calibre_library_path = format!("{}/Biblioteca do calibre/", home);
                let cli = Cli::backup_cli(calibre_library_path, Some("calibre".to_string()));
                backup(Some(cli));
            }
        }
    }
}
