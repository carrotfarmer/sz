use std::fs;
use std::path;

use ignore::WalkBuilder;

use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color, Panel,
    },
    Table,
};

use crate::Args;
use crate::utils::get_file_size;
use crate::{file::File, SortOpt};

pub fn print_dir_size_with_files(
    args: &mut Args,
    sort_opt: SortOpt,
) {
    clearscreen::clear().unwrap();

    let mut files = vec![];

    for result in WalkBuilder::new(&args.path)
        .hidden(!&args.include_hidden)
        .git_ignore(!&args.include_gitignored)
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let mut file_name = path.to_str().unwrap().to_string();

                    if file_name.len() > 35 {
                        file_name = file_name
                            .chars()
                            .rev()
                            .take(30)
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect::<String>();

                        file_name.insert_str(0, "...");
                    }

                    let file = File::new(file_name, get_file_size(path));

                    files.push(file);
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    match sort_opt {
        SortOpt::Asc => files.sort_by(|a, b| a.bytes().partial_cmp(&b.bytes()).unwrap()),
        SortOpt::Desc => files.sort_by(|a, b| b.bytes().partial_cmp(&a.bytes()).unwrap()),
        _ => (),
    }

    let file_len = files.len();

    if file_len > 50 && args.num_files.is_none() && !args.list_all {
        println!(
            "\x1b[1;33mwarning: {} files found, showing first 20\x1b[0m",
            file_len
        );
        args.num_files = Some(20);
    } else {
        args.num_files = Some(file_len);
    }

    let total_size = files
        .iter()
        .fold(0.0, |acc, file| acc + file.clone().bytes());

    if let Some(num_files) = args.num_files {
        files.truncate(num_files)
    }

    let total = File::new("TOTAL SIZE".to_string(), total_size);
    files.push(total);

    let mut style = RawStyle::from(Style::psql());

    style
        .set_color_top(Color::FG_MAGENTA)
        .set_color_bottom(Color::FG_MAGENTA)
        .set_color_left(Color::FG_MAGENTA)
        .set_color_right(Color::FG_MAGENTA)
        .set_color_corner_top_left(Color::FG_MAGENTA)
        .set_color_corner_top_right(Color::FG_MAGENTA)
        .set_color_corner_bottom_left(Color::FG_MAGENTA)
        .set_color_corner_bottom_right(Color::FG_MAGENTA)
        .set_color_intersection_bottom(Color::FG_MAGENTA)
        .set_color_intersection_top(Color::FG_MAGENTA)
        .set_color_intersection_right(Color::FG_MAGENTA)
        .set_color_intersection_left(Color::FG_MAGENTA)
        .set_color_intersection(Color::FG_MAGENTA)
        .set_color_horizontal(Color::FG_MAGENTA)
        .set_color_vertical(Color::FG_MAGENTA);


    let mut table = Table::new(&files);

    table
        .with(Panel::horizontal(
            files.len(),
            // make the separator magenta with escape sequences
            format!("\x1b[35m{}\x1b[0m", "-".repeat(table.total_width())),
        ))
        .with(Panel::footer(format!("\x1b[35m{}\x1b[0m files parsed", file_len)))
        .with(style);

    println!("{}", owo_colors::OwoColorize::bold(&table.to_string()));
}

pub fn print_dir_size(dir_path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
    let mut total_size = 0.0;

    for entry in WalkBuilder::new(&dir_path)
        .hidden(!include_hidden)
        .git_ignore(!include_gitignored)
        .build()
    {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    total_size += get_file_size(entry.path());
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
    let mut table = Table::new(&[dir_size]);
    let mut style = RawStyle::from(Style::extended());

    style
        .set_color_top(Color::FG_BRIGHT_BLUE)
        .set_color_bottom(Color::FG_BRIGHT_BLUE)
        .set_color_left(Color::FG_BRIGHT_BLUE)
        .set_color_right(Color::FG_BRIGHT_BLUE)
        .set_color_corner_top_left(Color::FG_BRIGHT_BLUE)
        .set_color_corner_top_right(Color::FG_BRIGHT_BLUE)
        .set_color_corner_bottom_left(Color::FG_BRIGHT_BLUE)
        .set_color_corner_bottom_right(Color::FG_BRIGHT_BLUE)
        .set_color_intersection_bottom(Color::FG_BRIGHT_BLUE)
        .set_color_intersection_top(Color::FG_BRIGHT_BLUE)
        .set_color_intersection_right(Color::FG_BRIGHT_BLUE)
        .set_color_intersection_left(Color::FG_BRIGHT_BLUE)
        .set_color_intersection(Color::FG_BRIGHT_BLUE)
        .set_color_horizontal(Color::FG_BRIGHT_BLUE)
        .set_color_vertical(Color::FG_BRIGHT_BLUE);

    table.with(style);

    println!(
        "{}",
        owo_colors::OwoColorize::white(&owo_colors::OwoColorize::bold(&table.to_string()))
    );
}
