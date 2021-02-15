use clap::Clap;
use std::{fs, path::PathBuf, process};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clap)]
#[clap(version = "1.0", author = "Dzefo")]
struct Opts {
    /// The Path which should be evaluated
    input: String,
}

fn main() -> Result<()> {
    use pretty_bytes::converter::convert;

    let opts: Opts = Opts::parse();

    let size;
    if fs::metadata(&opts.input)?.is_file() {
        size = get_file_size(&opts.input);
    } else {
        match get_dir_size(&opts.input) {
            Ok(s) => size = s,
            Err(_) => {
                println!("Konnte das Verzeichnis nicht lesen");
                process::exit(1);
            }
        }
    }
    println!("{}", convert(size as f64));

    Ok(())
}

fn get_file_size(path: &String) -> u64 {
    let meta = fs::metadata(path);
    match meta {
        Ok(m) => m.len(),
        Err(_) => 0,
    }
}

fn get_dir_size(path: impl Into<PathBuf>) -> Result<u64> {
    fn get_dir_size(mut dir: fs::ReadDir) -> Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => get_dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }
    get_dir_size(fs::read_dir(path.into())?)
}
