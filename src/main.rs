use clap::Parser;
use pretty_bytes::converter::{convert, Prefix};
use scale::{get_dir_size, get_file_size};
use std::{fs, process};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(name = "scale", version = "0.1.3")]
struct Args {
    /// The Path which should be evaluated
    input: String,
    /// Lists all the files with its size
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let opts = Args::parse();

    let size = if fs::metadata(&opts.input)?.is_file() {
        get_file_size(&opts.input)
    } else {
        match get_dir_size(&opts.input, opts.list) {
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
