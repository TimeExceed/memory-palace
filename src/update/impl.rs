use crate::{Item, file::*};
use log::*;
use std::path::PathBuf;

pub struct Update {
    pub into: Vec<PathBuf>,
    pub from: PathBuf,
}

impl Update {
    pub fn gogogo(self) {
        let mut items_into: Vec<_> = self.into.iter().map(|f| read_file(f)).collect();

        let items_from = read_file(&self.from);
        let mut remains = vec![];
        for item in items_from.into_iter() {
            if let Some(dest) = find_position(&mut items_into, &item) {
                *dest = item;
            } else {
                remains.push(item);
            }
        }

        for (f, items) in self.into.iter().zip(items_into.iter()) {
            debug!("write to {}", f.display());
            write_out(f, items);
        }
        debug!("write to {}", self.from.display());
        write_out(&self.from, &remains);
    }
}

fn find_position<'a>(items_into: &'a mut [Vec<Item>], item: &Item) -> Option<&'a mut Item> {
    let question = &item.question;
    let mut res = None;
    for items in items_into.iter_mut() {
        for item_in_items in items.iter_mut() {
            if &item_in_items.question == question {
                if res.is_none() {
                    res = Some(item_in_items);
                } else {
                    warn!("duplicate question: {}", question);
                    return None;
                }
            }
        }
    }
    if res.is_none() {
        warn!("No position to update: {}", question);
    }
    res
}
