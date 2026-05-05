use crate::models::Cli;

pub fn backup(cli: Cli) {
    if !cli.backup_path.is_dir() {
        panic!("backup is not a directory")
    }

    print!("backup ");
    dbg!(cli);
}
