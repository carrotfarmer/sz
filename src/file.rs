use std::fmt::Display;

use crate::file_size::FileSize;

use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color,
    },
    Table, Tabled,
};

#[derive(Tabled)]
pub struct File {
    file_name: String,
    file_size: FileSize,
}

impl File {
    pub fn new(file_name: String, file_size: f64) -> Self {
        let file_size = FileSize::new(file_size);

        Self {
            file_name,
            file_size,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style = RawStyle::from(Style::extended());

        style
            .set_color_top(Color::FG_BLUE)
            .set_color_bottom(Color::FG_BLUE)
            .set_color_left(Color::FG_BLUE)
            .set_color_right(Color::FG_BLUE)
            .set_color_corner_top_left(Color::FG_BLUE)
            .set_color_corner_top_right(Color::FG_BLUE)
            .set_color_corner_bottom_left(Color::FG_BLUE)
            .set_color_corner_bottom_right(Color::FG_BLUE)
            .set_color_intersection_bottom(Color::FG_BLUE)
            .set_color_intersection_top(Color::FG_BLUE)
            .set_color_intersection_right(Color::FG_BLUE)
            .set_color_intersection_left(Color::FG_BLUE)
            .set_color_intersection(Color::FG_BLUE)
            .set_color_horizontal(Color::FG_BLUE)
            .set_color_vertical(Color::FG_BLUE);

        let mut table = Table::new(&[self]);

        table.with(style);
        write!(f, "{}", table)?;

        Ok(())
    }
}
