use std::env;

use crate::{
    backup,
    models::{Cli, PredefinedFunc},
};

pub fn predefined(predefined_func: PredefinedFunc) {
    let home = env::var("HOME").expect("$HOME não existe");

    match predefined_func {
        PredefinedFunc::CalibreBackup => {
            // backup calibre library
            let calibre_library_path = format!("{}/Biblioteca do calibre/", home);
            backup(Some(Cli::backup_cli(
                calibre_library_path,
                Some("calibre_lib".to_string()),
            )));

            // backup calibre configs
            let calibre_config_path = format!("{}/.config/calibre/", home);
            backup(Some(Cli::backup_cli(
                calibre_config_path,
                Some("calibre_config".to_string()),
            )));
        }
    }
}
