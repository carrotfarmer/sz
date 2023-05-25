use crate::item::Item;

use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color, Panel,
    },
    Table,
};

use owo_colors::OwoColorize;

enum TableColor {
    Magenta,
    Blue,
}

impl TableColor {
    fn to_color(&self) -> Color {
        match self {
            TableColor::Magenta => Color::FG_BRIGHT_MAGENTA,
            TableColor::Blue => Color::FG_BRIGHT_BLUE,
        }
    }
}

fn gen_table_styles(table_color: TableColor) -> RawStyle {
    let mut style = RawStyle::from(Style::psql());

    let color = table_color.to_color();

    // I'm sorry Rustaceans
    style
        .set_color_top(color.clone())
        .set_color_bottom(color.clone())
        .set_color_left(color.clone())
        .set_color_right(color.clone())
        .set_color_corner_top_left(color.clone())
        .set_color_corner_top_right(color.clone())
        .set_color_corner_bottom_left(color.clone())
        .set_color_corner_bottom_right(color.clone())
        .set_color_intersection_bottom(color.clone())
        .set_color_intersection_top(color.clone())
        .set_color_intersection_right(color.clone())
        .set_color_intersection_left(color.clone())
        .set_color_intersection(color.clone())
        .set_color_horizontal(color.clone())
        .set_color_vertical(color.clone());

    style
}

pub fn print_table_files(files: Vec<Item>, items_len: usize, is_dir: bool) -> () {
    let style = gen_table_styles(TableColor::Magenta);

    let mut table = Table::new(&files);

    let msg;

    match is_dir {
        true => msg = format!("{} dirs parsed", items_len.to_string().magenta()),
        false => msg = format!("{} files parsed", items_len.to_string().magenta()),
    }

    table
        .with(Panel::horizontal(
            files.len(),
            format!("{}", "-".repeat(table.total_width()).magenta()),
        ))
        .with(Panel::footer(format!(
            "{}",
            msg
        )))
        .with(style);

    println!("{}", table.to_string().bold());
}

pub fn print_table_dir(dir_size: Item, dir_len: usize) -> () {
    let mut table = Table::new(&[dir_size]);
    let style = gen_table_styles(TableColor::Blue);

    let table = table.with(style);

    println!("{}", table.to_string().bold());
    println!("\n\n{} files parsed", dir_len.to_string().magenta().bold());
}

pub fn print_table_item(item: &Item) -> Table {
    let mut table = Table::new(&[item]);
    let style = gen_table_styles(TableColor::Blue);

    table.with(style);
    table
}
