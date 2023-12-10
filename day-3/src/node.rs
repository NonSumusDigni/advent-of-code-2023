use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum Node {
    Empty,
    Digit(char),
    Symbol(char),
}

impl Node {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '0'..='9' => Self::Digit(c),
            _ => Self::Symbol(c),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Digit(c) => write!(f, "{}", c),
            Self::Symbol(c) => write!(f, "{}", c),
        }
    }
}
