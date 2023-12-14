use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::gear::GearBuilder;

#[derive(Debug)]
pub struct PartNumberBuilder {
    x_range_incl: (usize, usize),
    y: usize,
    digits: Vec<char>,
    gears: Vec<Rc<RefCell<GearBuilder>>>,
    gear_locations: HashSet<(usize, usize)>,
    pub completed: bool,
    pub validated: bool,
}

impl PartNumberBuilder {
    pub fn new(x: usize, y: usize, digit: char) -> Self {
        Self {
            x_range_incl: (x, x),
            y,
            digits: vec![digit],
            gears: Vec::new(),
            gear_locations: HashSet::new(),
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

    pub fn add_gear(&mut self, gear: Rc<RefCell<GearBuilder>>, x: usize, y: usize) {
        if !self.gear_locations.contains(&(x, y)) {
            self.gear_locations.insert((x, y));
            self.gears.push(gear);
        }
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

    pub fn key(&self) -> (usize, usize) {
        (self.x_range_incl.0, self.y)
    }

    pub fn build(&self) -> u64 {
        if !self.buildable() {
            panic!("Cannot build part number")
        }

        let value = self
            .digits
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .expect("Failed to parse part number");

        self.gears.iter().for_each(|gear| {
            gear.borrow_mut().push(value);
        });

        value
    }
}

pub type PartNumbers = HashMap<(usize, usize), Rc<RefCell<PartNumberBuilder>>>;
