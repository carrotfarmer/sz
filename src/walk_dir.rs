use std::path;

use ignore::WalkBuilder;

use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color,
    },
    Table,
};

use crate::file::File;
use crate::utils::get_file_size;

pub fn print_file_size(path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
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

    println!("{}", table);
}
