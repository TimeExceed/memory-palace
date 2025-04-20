use crate::*;
use chrono::prelude::*;
use log::*;
use std::{cell::RefCell, rc::Rc};

pub struct Exam {
    /// the file of a memory palace.
    pub file_name: String,

    /// Take at most N items of things to remember.
    pub take: Option<usize>,

    /// Do everything except writing back.
    pub dry_run: bool,
}

impl Exam {
    pub fn gogogo(&self) {
        let items = read_file(&self.file_name);
        let now = Utc::now();
        let selected = exam::Selected::new(items, &now, self.take);
        let selected = Rc::new(RefCell::new(selected));
        exam::gui::App::start(&self.file_name, selected.clone());
        let items = selected.borrow_mut().feedback(&now);
        if self.dry_run {
            info!("dry run!");
        } else {
            write_out(&self.file_name, items);
        }
    }
}
