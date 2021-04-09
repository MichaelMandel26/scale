use std::cmp;
use std::{fs, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Prefix {
    Binary,
    Decimal,
}

pub fn convert(num: f64, prefix: Prefix) -> String {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }
    let delimiter: f64 = match prefix {
        Prefix::Binary => 1024_f64,
        Prefix::Decimal => 1000_f64,
    };
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap()
        * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{} {}", negative, pretty_bytes, unit)
}

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
