mod backup;
mod models;
mod predefined;
mod restore;

use crate::backup::backup;
use crate::models::{FuncType, PredefinedFunc};
use crate::predefined::predefined;
use crate::restore::restore;

fn main() {
    let func_arg = std::env::args().nth(1).expect("No function specified");
    let func = match func_arg.to_lowercase().as_str() {
        "backup" => FuncType::Backup,
        "restore" => FuncType::Restore,
        _ => panic!("Invalid function: {}. Use 'backup' or 'restore'", func_arg),
    };

    let predefined_func_arg = std::env::args()
        .nth(2)
        .expect("Needs at least a second argument");
    let predefined_func: Option<PredefinedFunc> = match predefined_func_arg.to_lowercase().as_str()
    {
        "calibre_backup" => Some(PredefinedFunc::CalibreBackup),
        _ => None,
    };

    match predefined_func {
        Some(p) => predefined(p),
        None => {
            match func {
                FuncType::Backup => backup(None),
                FuncType::Restore => restore(),
            }
            .unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
        }
    }
}
