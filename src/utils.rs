use std::io;
use std::path::Path;

use ignore::WalkBuilder;

use crate::Args;

pub fn get_file_size(path: &Path) -> f64 {
    match path.metadata() {
        Ok(metadata) => metadata.len() as f64,
        Err(e) => {
            println!("sz: error while reading path: {}", e);
            0.0
        }
    }
}

pub fn get_dir_size(path: &Path, args: Args) -> (f64, usize) {
    let mut dir_size: f64 = 0.0;
    let mut files_count = 0;

    for item in WalkBuilder::new(path)
        .hidden(!args.include_hidden)
        .git_exclude(!args.include_gitignored)
        .build()
    {
        match item {
            Ok(item) => {
                if item.path().is_file() {
                    dir_size += get_file_size(item.path());
                    files_count += 1;
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    (dir_size, files_count)
}

pub fn get_file_lines(path: &Path) -> usize {
    match std::fs::read_to_string(path) {
        Ok(content) => content.lines().count(),
        Err(e) => {
            println!("sz: error while reading path: {}", e);
            0
        }
    }
}

pub fn get_dir_lines(path: &Path, args: Args) -> usize {
    let mut lines = 0;

    for item in WalkBuilder::new(path)
        .hidden(!args.include_hidden)
        .git_exclude(!args.include_gitignored)
        .build()
    {
        match item {
            Ok(item) => {
                if item.path().is_file() {
                    lines += get_file_lines(item.path());
                }
            },

            Err(e) => println!("sz: error while reading path: {}", e)
        }
    }

    lines
}

pub fn shorten_name(item_name: String) -> String {
    if item_name.len() > 35 {
        let mut s = item_name
            .chars()
            .rev()
            .take(30)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        s.insert_str(0, "...");

        return s;
    }

    item_name
}

pub fn user_confirmation(message: String) -> Option<bool> {
    println!("{}", message);

    let mut proceed = String::from("");

    io::stdin()
        .read_line(&mut proceed)
        .expect("sz: error while reading input");

    match proceed.as_str() {
        "y\n" => Some(true),
        "n\n" => Some(false),
        _ => None,
    }
}
