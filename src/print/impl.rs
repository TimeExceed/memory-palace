use super::*;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Print {
    Typst { input: PathBuf, output: PathBuf },
}

impl Print {
    pub fn gogogo(&self) {
        match self {
            Print::Typst { input, output } => {
                typst(input, output);
            }
        }
    }
}
