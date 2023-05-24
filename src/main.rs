mod item;
mod item_size;
mod sort_opt;
mod table;
mod utils;
mod walk_dir;

use clap::Parser;
use owo_colors::OwoColorize;

use std::fs;
use std::path;

use std::io::ErrorKind;

use crate::item::Item;
use crate::sort_opt::SortOpt;
use crate::utils::get_file_size;
use crate::walk_dir::{print_dir_size, print_dir_size_with_files};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Path to file or directory
    path: path::PathBuf,

    /// Sort files by size in descending order (with -l flag)
    #[clap(short = 'd', long)]
    sort_files_desc: bool,

    /// Sort files by size in ascending order (with -l flag)
    #[clap(short = 'a', long)]
    sort_files_asc: bool,

    /// Include hidden files
    #[clap(short = 'i', long)]
    include_hidden: bool,

    /// List the files in the directory
    #[clap(short = 'l', long)]
    list_files: bool,

    /// List all files in the directory
    /// even if directory count exceeds 50
    #[clap(short = 'L', long)]
    list_all: bool,

    /// Include gitignored files
    #[clap(short = 'g', long)]
    include_gitignored: bool,

    /// Directories to exclude
    #[clap(short = 'e', long, num_args=1.., value_delimiter=' ')]
    exclude_dirs: Vec<path::PathBuf>,

    /// Number of files to list
    #[clap(short = 'n', long)]
    num_files: Option<usize>,

    /// Show only directories
    #[clap(short = 'D', long)]
    only_dirs: bool,

    /// Recursively list directories
    /// Only to be used with -D flag 
    #[clap(short = 'R', long)]
    recursive_dirs: bool,

    /// Show lines for items
    #[clap(short = 's', long)]
    show_lines: bool,
}

fn main() {
    let mut args = Args::parse();
    if args.path.is_dir() {
        let buf = fs::read_dir(&args.path);
        if let Some(num) = args.num_files {
            if num <= 0 || num > 100 {
                println!("{}", "sz: error: invalid number of files to list".red());
                println!("{}", "sz: number of files must be between 1 and 100".blue());
                return;
            }
        }
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

        if args.list_files || args.list_all {
            let sort_opt = SortOpt::from_args(&args);

            print_dir_size_with_files(&mut args, sort_opt);
        } else {
            print_dir_size(args.path, args.include_hidden, args.include_gitignored);
        }
    } else {
        let file = Item::new(
            String::from(args.path.file_name().unwrap().to_str().unwrap()),
            get_file_size(args.path.as_path()),
        );

        println!("{}", file);
    }
}
