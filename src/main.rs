mod file;
mod file_size;
mod utils;
mod walk_dir;

use clap::Parser;
use file::File;
use utils::get_file_size;

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

    if args.path.is_dir() {
        let buf = fs::read_dir(&args.path);

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

        if args.list_all {
            print_file_size(args.path, args.include_hidden, args.include_gitignored);
        } else {
            if let None = args.path.file_name() {
                println!("sz: could not read file: {}", args.path.display());
            }

            let file_name = String::from(args.path.file_name().unwrap().to_str().unwrap());

            let file = File::new(file_name, get_file_size(args.path.as_path()));
            println!("{}", file);
        }
    } else {
        let file = File::new(
            String::from(args.path.file_name().unwrap().to_str().unwrap()),
            get_file_size(args.path.as_path()),
        );
        println!("{}", file);
    }
}
