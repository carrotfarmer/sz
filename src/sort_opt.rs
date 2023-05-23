use crate::Args;
use crate::item::Item;

pub enum SortOpt {
    Asc,
    Desc,
    Def,
}

impl SortOpt {
    pub fn from_args(args: &Args) -> SortOpt {
        if args.sort_files_asc {
            return SortOpt::Asc;
        } else if args.sort_files_desc {
            return SortOpt::Desc;
        } else {
            return SortOpt::Def;
        }
    }

    pub fn sort_items(&self, items: &mut Vec<Item>) -> () {
        match self {
            SortOpt::Asc => items.sort_by(|a, b| a.bytes().partial_cmp(&b.bytes()).unwrap()),
            SortOpt::Desc => items.sort_by(|a, b| b.bytes().partial_cmp(&a.bytes()).unwrap()),
            SortOpt::Def => (),
        }
    }
}
