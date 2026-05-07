use std::fmt::format;
use std::fs::File;
use walkdir::WalkDir;

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::models::Cli;

pub fn backup(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if !cli.backup_path.is_dir() {
        panic!("Backup is not a directory")
    }

    if cli.restore_path.exists() {
        panic!("Restore file already exists");
    }

    let file = File::create(cli.restore_path.clone())?;
    let walkdir = WalkDir::new(cli.backup_path.clone());

    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    for entry_result in walkdir.into_iter() {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                return Err(format!("Error while traversing directory: {e}").into());
            }
        };

        let path = entry.path();
        let path_striped = path.strip_prefix(cli.backup_path.clone())?;
        let path_as_string = path_striped
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| format!("{:?}", path_striped.display()))?;

        if path.is_file() {
            zip.start_file(path_as_string, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        } else if !path_striped.as_os_str().is_empty() {
            zip.add_directory(path_as_string, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
