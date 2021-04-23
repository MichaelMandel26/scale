use pretty_bytes::converter::{convert, Prefix};
use scale::{get_dir_size, get_file_size};
use std::{fs, process};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt, Debug)]
#[structopt(name = "scale", version = "0.1.3")]
struct Opt {
    /// The Path which should be evaluated
    input: String,
    /// Lists all the files with its size
    #[structopt(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();
    // Add option to just use current directory
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
