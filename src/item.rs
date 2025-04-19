use crate::UtcTime;
use log::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Item {
    pub question: String,
    pub answer: String,
    pub first_remember_time: Option<UtcTime>,
    pub last_check_time: Option<UtcTime>,
    pub due_time: Option<UtcTime>,
    pub tag: Option<String>,
}

const MAX_DURATION: chrono::Duration = chrono::Duration::days(360);
const INIT_DURATION: chrono::Duration = chrono::Duration::hours(20);

impl Item {
    pub fn correct(&mut self, now: &UtcTime) {
        info!("correct: {}", self.question);
        if let Some(ref first_remember_time) = self.first_remember_time {
            let timeout = timeout(&(*now - *first_remember_time));
            self.due_time.replace(*now + timeout);
        } else {
            self.first_remember_time = Some(*now);
            let timeout = timeout(&INIT_DURATION);
            self.due_time.replace(*now + timeout);
        }
        self.last_check_time.replace(*now);
    }

    pub fn wrong(&mut self, now: &UtcTime) {
        warn!("  wrong: {}", self.question);
        self.first_remember_time = None;
        let timeout = timeout(&INIT_DURATION);
        self.due_time.replace(*now + timeout);
        self.last_check_time.replace(*now);
    }
}

fn timeout(delay: &chrono::Duration) -> chrono::Duration {
    let int_dur = delay.min(&MAX_DURATION).num_seconds();
    let mut rng = rand::rng();
    let timeout = rng.random_range(int_dur..(int_dur + (int_dur / 2)));
    chrono::Duration::seconds(timeout)
}

pub struct Selected {
    items: Vec<Item>,
    selected_and_correctness: Vec<(usize, bool)>,
}

impl Selected {
    pub fn new(items: Vec<Item>, now: &UtcTime, take: Option<usize>) -> Self {
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
        let mut rng = rand::rng();
        selected_and_correctness.shuffle(&mut rng);
        if let Some(n) = take {
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

    pub fn write_out(&self, file: &str) {
        let out: Vec<_> = self
            .selected_and_correctness
            .iter()
            .map(|x| SelectedItemInDisk {
                index: x.0,
                correct: x.1,
            })
            .collect();
        let out = SelectedInDisk {
            selected_items: out,
        };
        let content = toml::to_string_pretty(&out).unwrap();
        std::fs::write(file, content).unwrap();
    }

    pub fn read_back(items: Vec<Item>, file: &str) -> Self {
        let content = std::fs::read(file).unwrap();
        let content = std::str::from_utf8(&content).unwrap();
        let selected_items: SelectedInDisk = toml::from_str(content).unwrap();
        let selected_and_correctness: Vec<_> = selected_items
            .selected_items
            .iter()
            .map(|x| (x.index, x.correct))
            .collect();
        info!(
            "{}/{} items selected.",
            selected_and_correctness.len(),
            items.len()
        );
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

#[derive(Debug, Serialize, Deserialize)]
struct SelectedItemInDisk {
    index: usize,
    correct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct SelectedInDisk {
    selected_items: Vec<SelectedItemInDisk>,
}
