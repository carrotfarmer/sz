mod utils;
mod walk_dir;
mod file;

use clap::Parser;

use std::fs;
use std::path;

use std::io::ErrorKind;

use crate::walk_dir::print_file_size;

#[derive(Parser, Debug)]
struct Args {
    path: path::PathBuf,

    #[clap(short = 's', long)]
    sort_files: bool,

    #[clap(short = 'i', long)]
    include_hidden: bool,

    #[clap(short = 'l', long)]
    list_all: bool,

    #[clap(short = 'g', long)]
    include_gitignored: bool,
}

fn main() {
    let args = Args::parse();

    let buf = fs::read_dir(&args.path);

    match (args.path.is_dir(), args.path.is_file()) {
        (true, false) => println!("{} is a directory", args.path.display()),
        (false, true) => println!("{} is a file", args.path.display()),
        _ => {
            if let Err(e) = buf {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!(
                            "sz: error while reading path: `{}` not found",
                            args.path.display()
                        );
                    }

                    _ => println!("sz: error while reading path: {}", e),
                }
            }
        }
    }

    if args.list_all {
        print_file_size(args.path, args.include_hidden);
    }
}
