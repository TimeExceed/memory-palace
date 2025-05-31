use crate::*;
use chrono::prelude::*;
use log::*;
use rand::prelude::*;
use std::{collections::*, path::PathBuf};

#[derive(Debug)]
pub struct Select {
    /// the file to be selected from
    pub input: PathBuf,

    /// the file to be wrote out
    pub output: PathBuf,

    /// Selects at most N items of things.
    pub take: Option<usize>,

    /// Selects only timed-out items
    pub timeout: bool,

    /// Selects only items with one of the specified tags
    pub tags: Option<HashSet<String>>,
}

impl Select {
    pub fn gogogo(&self) {
        let mut items = read_file(&self.input);
        if self.timeout {
            let now = Utc::now();
            let retain_fn = |x: &Item| -> bool {
                if let Some(due) = x.due_time {
                    due < now
                } else {
                    true
                }
            };
            items.retain(|x| {
                let res = retain_fn(x);
                if !res {
                    debug!("exclude item because of not timing out: {}", x.question);
                }
                res
            });
        }
        if let Some(tags) = &self.tags {
            let retain_fn = |x: &Item| -> bool {
                let Some(item_tag) = &x.tag else {
                    return false;
                };
                for x in item_tag.split(" ") {
                    if tags.contains(x) {
                        return true;
                    }
                }
                false
            };
            items.retain(|x| {
                let res = retain_fn(x);
                if !res {
                    debug!("exclude item because of no tags: {}", x.question);
                }
                res
            });
        }
        if let Some(n) = self.take {
            info!("shuffle and take {n} out of {} items.", items.len());
            let mut rng = rand::rng();
            items.shuffle(&mut rng);
            items.truncate(n);
        } else {
            info!("take all {} items.", items.len());
        }
        append(&self.output, items);
    }
}
