use std::{
    error::Error,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "file", help = "File to execute")]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if !Path::new(&args.file).is_file() {
        return Err(format!("file {} does not exist", &args.file.to_str().unwrap()).into());
    }

    Ok(())
}
