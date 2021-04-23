use pretty_bytes::converter::{convert, Prefix};
use std::{fs, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_file_size(path: &String) -> u64 {
    let meta = fs::metadata(path);
    match meta {
        Ok(m) => m.len(),
        Err(_) => 0,
    }
}

pub fn get_dir_size(path: impl Into<PathBuf>, list: bool) -> Result<u64> {
    fn get_dir_size(mut dir: fs::ReadDir, list: bool) -> Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => get_dir_size(fs::read_dir(file.path())?, list)?,
                data => data.len(),
            };
            if list {
                println!(
                    "{:indent$} {}",
                    convert(size as f64, Prefix::Binary),
                    file.path().to_string_lossy(),
                    indent = 10
                );
            }
            Ok(acc + size)
        })
    }
    get_dir_size(fs::read_dir(path.into())?, list)
}
