use clap::Clap;
use scale::{convert, get_dir_size, get_file_size, Prefix};
use std::{fs, process};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clap)]
#[clap(version = "0.1.1", author = "Dzefo")]
struct Opts {
    /// The Path which should be evaluated
    input: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    // Add option to just use current directory
    let size = if fs::metadata(&opts.input)?.is_file() {
        get_file_size(&opts.input)
    } else {
        match get_dir_size(&opts.input) {
            Ok(s) => s,
            Err(_) => {
                println!("Could not get the directory size");
                process::exit(1);
            }
        }
    };

    println!("{}", convert(size as f64, Prefix::Binary));

    Ok(())
}
