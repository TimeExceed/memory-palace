use super::*;

#[derive(Debug)]
pub enum Print {
    Typst {
        input: String,
        output: String,
    },
}

impl Print {
    pub fn gogogo(&self) {
        match self {
            Print::Typst{input, output} => {
                typst(input, output);
            }
        }
    }
}
