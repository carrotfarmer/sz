use std::fmt::Display;

use crate::{item_size::ItemSize, table::print_table_item};

use owo_colors::OwoColorize;
use tabled::Tabled;

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
        let table = print_table_item(self);
        write!(f, "{}", table.bold())?;

        Ok(())
    }
}
