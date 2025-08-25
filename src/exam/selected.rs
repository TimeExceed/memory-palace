use crate::{Item, UtcTime};
use super::r#impl::Exam;
use log::*;
use rand::prelude::*;

pub struct Selected {
    items: Vec<Item>,
    selected_and_correctness: Vec<(usize, bool)>,
}

impl Selected {
    pub fn new(items: Vec<Item>, now: &UtcTime, cfg: &Exam) -> Self {
        let mut selected_and_correctness: Vec<_> = items
            .iter()
            .enumerate()
            .filter(|(_, x)| {
                if let Some(due) = x.due_time {
                    due < *now
                } else {
                    true
                }
            })
            .map(|(i, _)| (i, true))
            .collect();
        if cfg.sort {
            selected_and_correctness.sort_by_key(|x| &items[x.0].question);
        } else {
            let mut rng = rand::rng();
            selected_and_correctness.shuffle(&mut rng);
        }
        if let Some(n) = cfg.take {
            info!("{}/{} items selected.", n, selected_and_correctness.len());
            selected_and_correctness.truncate(n);
        } else {
            info!(
                "{}/{} items selected.",
                selected_and_correctness.len(),
                selected_and_correctness.len()
            );
        }
        Self {
            items,
            selected_and_correctness,
        }
    }

    pub fn feedback(&mut self, now: &UtcTime) -> Vec<Item> {
        let mut res = vec![];
        std::mem::swap(&mut res, &mut self.items);
        for (i, c) in self.selected_and_correctness.iter() {
            if *c {
                res[*i].correct(now);
            } else {
                res[*i].wrong(now);
            }
        }
        res.sort_by_cached_key(|x| x.question.clone());
        res
    }

    pub fn items(&self) -> Vec<(Item, bool)> {
        self.selected_and_correctness
            .iter()
            .map(|(x, c)| (self.items[*x].clone(), *c))
            .collect()
    }

    pub fn set(&mut self, idx: usize) {
        self.selected_and_correctness[idx].1 = true;
    }

    pub fn unset(&mut self, idx: usize) {
        self.selected_and_correctness[idx].1 = false;
    }
}
