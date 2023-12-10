use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct PartNumberBuilder {
    x_range_incl: (usize, usize),
    y: usize,
    digits: Vec<char>,
    pub completed: bool,
    pub validated: bool,
}

impl PartNumberBuilder {
    pub fn new(x: usize, y: usize, digit: char) -> Self {
        Self {
            x_range_incl: (x, x),
            y,
            digits: vec![digit],
            completed: false,
            validated: false,
        }
    }

    pub fn push(&mut self, digit: char) {
        self.x_range_incl.1 += 1;
        self.digits.push(digit);
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn validate(&mut self) {
        self.validated = true;
    }

    pub fn nodes(&self) -> Vec<(usize, usize)> {
        let mut nodes = Vec::new();
        for x in self.x_range_incl.0..=self.x_range_incl.1 {
            nodes.push((x, self.y));
        }
        nodes
    }

    pub fn buildable(&self) -> bool {
        self.completed && self.validated
    }

    pub fn build(&self) -> u64 {
        if !self.buildable() {
            panic!("Cannot build part number")
        }

        self.digits
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .expect("Failed to parse part number")
    }
}

pub type UnvalidatedPartNumbers = HashMap<(usize, usize), Rc<RefCell<PartNumberBuilder>>>;
