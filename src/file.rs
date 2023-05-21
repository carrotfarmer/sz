use std::fmt::Display;

use crate::file_size::FileSize;

use owo_colors::OwoColorize;
use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color,
    },
    Table, Tabled,
};

#[derive(Tabled, Clone)]
pub struct File {
    file_name: String,
    file_size: FileSize,
    
    #[tabled(skip)]
    bytes: f64,
}

impl File {
    pub fn new(file_name: String, file_size_bytes: f64) -> Self {
        let file_size = FileSize::new(file_size_bytes);

        Self {
            file_name,
            file_size,
            bytes: file_size_bytes,
        }
    }

    pub fn file_name(self) -> String {
        self.file_name
    }

    pub fn file_size(self) -> FileSize {
        self.file_size
    }

    pub fn bytes(&self) -> f64 {
        self.bytes
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
        write!(f, "{}", table.bold())?;

        Ok(())
    }
}
