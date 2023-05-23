use std::fmt::Display;

use crate::item_size::ItemSize;

use owo_colors::OwoColorize;
use tabled::{
    settings::{
        style::{RawStyle, Style},
        Color,
    },
    Table, Tabled,
};

#[derive(Tabled, Clone, PartialEq)]
pub struct Item {
    #[tabled(rename = "file/dir name")]
    item_name: String,

    #[tabled(rename = "size")]
    item_size: ItemSize,
    
    #[tabled(skip)]
    bytes: f64,
}

impl Item {
    pub fn new(item_name: String, item_size_bytes: f64) -> Self {
        let item_size = ItemSize::new(item_size_bytes);

        Self {
            item_name,
            item_size,
            bytes: item_size_bytes,
        }
    }

    pub fn item_name(self) -> String {
        self.item_name
    }

    pub fn item_size(self) -> ItemSize {
        self.item_size
    }

    pub fn bytes(&self) -> f64 {
        self.bytes
    }
}

impl Display for Item {
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
