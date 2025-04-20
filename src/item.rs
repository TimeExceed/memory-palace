use crate::UtcTime;
use log::*;
use rand::prelude::*;

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
