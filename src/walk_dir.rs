use std::path;

use ignore::WalkBuilder;

use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color,
    },
    Table,
};

use crate::{file::File, SortOpt};
use crate::{file_size, utils::get_file_size};

pub fn print_file_size(
    path: path::PathBuf,
    include_hidden: bool,
    include_gitignored: bool,
    sort_opt: SortOpt,
) {
    let mut files = vec![];

    for result in WalkBuilder::new(path)
        .hidden(!include_hidden)
        .git_ignore(!include_gitignored)
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let file_name = path.to_str().unwrap().to_string();
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

    let total_size = files
        .iter()
        .fold(0.0, |acc, file| acc + file.clone().bytes());
    let total = File::new("TOTAL SIZE".to_string(), total_size);
    files.push(total);

    let mut style = RawStyle::from(Style::extended());

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
    table.with(style);
    println!("{}", owo_colors::OwoColorize::bold(&table.to_string()));
}

pub fn print_dir_size(dir_path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
    let mut total_size = 0.0;

    for entry in WalkBuilder::new(dir_path)
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

    let dir_size = File::new("Total".to_string(), total_size);
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

    println!("{}", owo_colors::OwoColorize::bold(&table.to_string()));
}
