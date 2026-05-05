use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FuncType {
    Backup,
    Restore,
}

fn parse_func_type(s: &str) -> Result<FuncType, String> {
    match s.to_lowercase().as_str() {
        "backup" => Ok(FuncType::Backup),
        "restore" => Ok(FuncType::Restore),
        _ => Err(format!(
            "Invalid function: {}. Use 'backup' or 'restore'",
            s
        )),
    }
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub func: FuncType,
    pub backup_path: PathBuf,
    pub restore_path: PathBuf,
}
