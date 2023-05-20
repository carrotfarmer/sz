use clap::Parser;

use std::fs;
use std::path;

use std::io::ErrorKind;

#[derive(Parser, Debug)]
struct Args {
    path: path::PathBuf,
}

fn main() {
    let args = Args::parse();

    match (args.path.is_dir(), args.path.is_file()) {
        (true, false) => println!("{} is a directory", args.path.display()),
        (false, true) => println!("{} is a file", args.path.display()),
        _ => {
            if let Err(e) = fs::read_dir(&args.path) {
                match e.kind() {
                    ErrorKind::NotFound => println!(
                        "sz: error while reading path: `{}` not found",
                        args.path.display()
                    ),
                    _ => println!("sz: error while reading path: {}", e),
                }
            }
        }
    }
}
