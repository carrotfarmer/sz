use std::fs;
use std::path;

use ignore::WalkBuilder;

use crate::table::{print_table_dir, print_table_files};
use crate::utils::{get_dir_size, get_file_size};
use crate::{file::File, Args, SortOpt};

pub fn print_dir_size_with_files(args: &mut Args, sort_opt: SortOpt) {
    clearscreen::clear().unwrap();

    let mut items = vec![];

    for result in WalkBuilder::new(&args.path)
        .hidden(!&args.include_hidden)
        .git_ignore(!&args.include_gitignored)
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if args.only_dirs {
                    if path.is_dir() {
                        let dir_name = path.to_str().unwrap().to_string();
                        let mut dir_name_to_display = dir_name.clone();

                        if dir_name.len() > 35 {
                            dir_name_to_display = dir_name
                                .chars()
                                .rev()
                                .take(30)
                                .collect::<String>()
                                .chars()
                                .rev()
                                .collect::<String>();

                            dir_name_to_display.insert_str(0, "...");
                        }

                        if path.parent() == Some(&args.path) {
                            if args.exclude_dirs.is_empty() {
                                let dir = File::new(
                                    dir_name_to_display.clone(),
                                    get_dir_size(path, args.clone()),
                                );

                                items.push(dir);
                            } else {
                                for dir in &args.exclude_dirs {
                                    if !dir_name.contains(dir.to_str().unwrap()) {
                                        let dir = File::new(
                                            dir_name_to_display.clone(),
                                            get_dir_size(path, args.clone()),
                                        );

                                        items.push(dir);
                                    }
                                }
                            }
                        }
                    }
                } else if path.is_file() {
                    let file_name = path.to_str().unwrap().to_string();
                    let mut file_name_to_display = file_name.clone();

                    if file_name.len() > 35 {
                        file_name_to_display = file_name
                            .chars()
                            .rev()
                            .take(30)
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect::<String>();

                        file_name_to_display.insert_str(0, "...");
                    }

                    if args.exclude_dirs.is_empty() {
                        let file = File::new(file_name_to_display.clone(), get_file_size(path));

                        items.push(file);
                    } else {
                        for dir in &args.exclude_dirs {
                            if !file_name.contains(dir.to_str().unwrap()) {
                                let file =
                                    File::new(file_name_to_display.clone(), get_file_size(path));

                                items.push(file);
                            }
                        }
                    }
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    match sort_opt {
        SortOpt::Asc => items.sort_by(|a, b| a.bytes().partial_cmp(&b.bytes()).unwrap()),
        SortOpt::Desc => items.sort_by(|a, b| b.bytes().partial_cmp(&a.bytes()).unwrap()),
        _ => (),
    }

    let item_len = items.len();

    if item_len > 50 && args.num_files.is_none() && !args.list_all {
        println!(
            "\x1b[1;33mwarning: {} items found, showing first 20\x1b[0m",
            item_len
        );
        args.num_files = Some(20);
    } else {
        args.num_files = Some(item_len);
    }

    let total_size = items
        .iter()
        .fold(0.0, |acc, item| acc + item.clone().bytes());

    if let Some(num_files) = args.num_files {
        items.truncate(num_files)
    }

    let total = File::new("TOTAL SIZE".to_string(), total_size);
    items.push(total);

    print_table_files(items, item_len);
}

pub fn print_dir_size(dir_path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
    let mut total_size = 0.0;
    let mut total_files_parsed: usize = 0;

    for entry in WalkBuilder::new(&dir_path)
        .hidden(!include_hidden)
        .git_ignore(!include_gitignored)
        .build()
    {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    total_size += get_file_size(entry.path());
                    total_files_parsed += 1;
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    let dir_size = File::new(
        if let Some(_) = dir_path.file_name() {
            fs::canonicalize(dir_path)
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        } else {
            fs::canonicalize(dir_path)
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        },
        total_size,
    );

    print_table_dir(dir_size, total_files_parsed);
}
