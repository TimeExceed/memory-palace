use crate::UtcTime;
use log::*;
use rand::prelude::*;
use static_init::dynamic;

#[derive(Debug, Clone)]
pub struct Item {
    pub question: String,
    pub answer: String,
    pub last_remember_time: Option<UtcTime>,
    pub last_check_time: Option<UtcTime>,
    pub due_time: Option<UtcTime>,
    pub duration: Option<chrono::Duration>,
    pub tag: Option<String>,
}

#[dynamic]
static MAX_DURATION: chrono::Duration = chrono::Duration::days(180);
#[dynamic]
static INIT_DURATION: chrono::Duration = chrono::Duration::hours(8);

impl Item {
    pub fn correct(&mut self, now: &UtcTime) {
        info!("correct: {}", self.question);
        self.last_remember_time.replace(*now);
        if let Some(ref mut dur) = self.duration {
            *dur = *dur + *dur;
            if *dur > *MAX_DURATION {
                *dur = *MAX_DURATION;
            }
        } else {
            self.duration.replace(*INIT_DURATION);
        }
        let timeout = self.timeout();
        self.due_time.replace(*now + timeout);
        self.last_check_time.replace(*now);
    }

    pub fn timeout(&mut self) -> chrono::Duration {
        let int_dur = self.duration.unwrap().num_seconds();
        let mut rng = rand::thread_rng();
        let timeout = rng.gen_range((int_dur / 2)..int_dur);
        chrono::Duration::seconds(timeout)
    }

    pub fn wrong(&mut self, now: &UtcTime) {
        warn!("  wrong: {}", self.question);
        self.duration.replace(*INIT_DURATION);
        let timeout = self.timeout();
        self.due_time.replace(*now + timeout);
        self.last_check_time.replace(*now);
    }
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
        let mut rng = rand::thread_rng();
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
