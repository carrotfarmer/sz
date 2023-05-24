use std::fs;
use std::path;

use ignore::WalkBuilder;

use crate::sort_opt::SortOpt;
use crate::table::{print_table_dir, print_table_files};
use crate::utils::{get_dir_size, get_file_size, get_file_lines, shorten_name, user_confirmation};
use crate::{item::Item, Args};

pub fn print_dir_size_with_files(args: &mut Args, sort_opt: SortOpt) {
    match clearscreen::clear() {
        Ok(_) => {}
        Err(e) => println!("sz: error while clearing screen: {}", e),
    }

    let mut items = vec![];
    let mut files_count = 0;
    let mut lines = 0;

    for result in WalkBuilder::new(&args.path)
        .hidden(!&args.include_hidden)
        .git_ignore(!&args.include_gitignored)
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();

                let condition = match args.recursive_dirs {
                    true => &args.path != path && path.is_dir(),
                    false => path.parent() == Some(&args.path),
                };

                if args.only_dirs {
                    if path.is_dir() {
                        let dir_name = path.to_str().unwrap().to_string();
                        let dir_name_to_display = shorten_name(dir_name.clone());

                        if condition {
                            if !args.exclude_dirs.is_empty()
                                && args.exclude_dirs.contains(&path.to_path_buf())
                            {
                                continue;
                            }

                            if args.show_lines {
                                lines += get_file_lines(path)
                            }

                            let (file_size, fc) = get_dir_size(path, args.clone());
                            let dir = Item::new(dir_name_to_display.clone(), file_size);

                            files_count += fc;
                            items.push(dir);
                        }
                    }
                } else if path.is_file() {
                    let file_name = path.to_str().unwrap().to_string();
                    let file_name_to_display = shorten_name(file_name.clone());

                    if !args.exclude_dirs.is_empty()
                        && args
                            .exclude_dirs
                            .contains(&path.parent().unwrap().to_path_buf())
                    {
                        continue;
                    }

                    if args.show_lines {
                        lines += get_file_lines(path)
                    }

                    let file = Item::new(file_name_to_display.clone(), get_file_size(path));
                    files_count += 1;
                    items.push(file);
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    sort_opt.sort_items(&mut items);
    let item_len = items.len();

    if args.num_files == None {
        if item_len > 50 && args.num_files.is_none() && !args.list_all {
            println!(
                "\x1b[1;33mwarning: {} items found, showing first 20\x1b[0m",
                item_len
            );

            args.num_files = Some(20);
        } else if args.list_all {
            if item_len > 50 {
                let msg = format!(
                    "\x1b[1;33mwarning: {} items found, proceed? [y/n]\x1b[0m",
                    item_len
                );

                let confirmation = user_confirmation(msg);

                match confirmation {
                    Some(val) => match val {
                        true => args.num_files = Some(item_len),
                        false => args.num_files = Some(50),
                    },

                    None => {
                        println!("sz: invalid input");
                        std::process::exit(1);
                    }
                }
            }
        } else {
            args.num_files = Some(item_len);
        }
    }

    let total_size = items
        .iter()
        .fold(0.0, |acc, item| acc + item.clone().bytes());

    if let Some(num_files) = args.num_files {
        items.truncate(num_files)
    }

    let total = Item::new("TOTAL SIZE".to_string(), total_size);
    items.push(total);

    print_table_files(items, files_count);

    if args.show_lines {
        println!("\n\x1b[1;33mTotal lines: {}\x1b[0m", lines);
    }
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

    let dir_size = Item::new(
        if let Some(_) = dir_path.file_name() {
            fs::canonicalize(dir_path)
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        } else {
            dir_path.to_str().unwrap().to_string()
        },
        total_size,
    );

    print_table_dir(dir_size, total_files_parsed);
}
