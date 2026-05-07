use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FuncType {
    Backup,
    Restore,
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub func: FuncType,
    pub backup_path: PathBuf,
    pub restore_path: PathBuf,
}
