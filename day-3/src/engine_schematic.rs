use std::cell::RefCell;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use crate::node::Node;
use crate::part_number::{PartNumberBuilder, UnvalidatedPartNumbers};

pub struct EngineSchematic {
    pub part_numbers: Vec<u64>,
    nodes: Vec<Vec<Node>>,
}

impl EngineSchematic {
    pub fn try_from_file<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(path)?);

        let mut nodes = Vec::new();
        let mut part_numbers = Vec::new();

        let mut unvalidated_part_numbers = UnvalidatedPartNumbers::new();
        let mut symbols: HashSet<(usize, usize)> = HashSet::new();

        let mut current_part_number_builder: Option<Rc<RefCell<PartNumberBuilder>>> = None;

        for (y, line) in reader.lines().enumerate() {
            let mut row = Vec::new();
            for (x, character) in line?.chars().enumerate() {
                println!("\n\n===== ROW {y}, CHARACTER {x}: '{character}' =====\n\n");
                let node = Node::new(character);

                match node {
                    Node::Empty => {}
                    Node::Digit(_) => {
                        if let Some(part_number_builder) = &current_part_number_builder {
                            part_number_builder.borrow_mut().push(character);
                        } else {
                            let part_number_builder =
                                Rc::new(RefCell::new(PartNumberBuilder::new(x, y, character)));
                            current_part_number_builder = Some(part_number_builder.clone());
                        }

                        let part_number_builder = current_part_number_builder
                            .as_ref()
                            .expect("Unreachable: No part number");

                        if !part_number_builder.borrow().validated {
                            if compute_back_neighbors(x, y)
                                .iter()
                                .any(|n| symbols.contains(n))
                            {
                                part_number_builder.borrow_mut().validate();
                                if part_number_builder.borrow().buildable() {
                                    let built = part_number_builder.borrow().build();
                                    println!(
                                        "Built: {}, current char: {}, code loc: Node::Digit",
                                        built, character
                                    );
                                    part_numbers.push(built);
                                    part_number_builder.borrow().nodes().iter().for_each(|n| {
                                        if n != &(x, y) {
                                            unvalidated_part_numbers.remove(n);
                                        }
                                    });
                                }
                            } else {
                                unvalidated_part_numbers
                                    .insert((x, y), part_number_builder.clone());
                            }
                        }
                    }
                    Node::Symbol(_) => {
                        symbols.insert((x, y));
                        compute_back_neighbors(x, y).iter().for_each(|n| {
                            if let Some(part_number_builder) =
                                unvalidated_part_numbers.get(n).cloned()
                            {
                                part_number_builder.borrow_mut().validate();
                                if part_number_builder.borrow().buildable() {
                                    let built = part_number_builder.borrow().build();
                                    println!(
                                        "Built: {}, current char: {}, code loc: Node::Symbol",
                                        built, character
                                    );
                                    part_numbers.push(built);
                                    part_number_builder.borrow().nodes().iter().for_each(|n| {
                                        unvalidated_part_numbers.remove(n);
                                    });
                                }
                            }
                        });
                    }
                }

                if !matches!(node, Node::Digit(_)) {
                    if let Some(part_number_builder) = current_part_number_builder {
                        part_number_builder.borrow_mut().complete();
                        if part_number_builder.borrow().buildable() {
                            let built = part_number_builder.borrow().build();
                            println!(
                                "Built: {}, current char: {}, code loc: !matches",
                                built, character
                            );
                            part_numbers.push(built);
                            part_number_builder.borrow().nodes().iter().for_each(|n| {
                                unvalidated_part_numbers.remove(n);
                            });
                        }
                        current_part_number_builder = None;
                    }
                }

                println!("AFTER:\n");
                println!(
                    "Current part number builder: {:?}",
                    current_part_number_builder
                );
                println!(
                    "Unvalidated part numbers: {:?}",
                    unvalidated_part_numbers
                        .keys()
                        .into_iter()
                        .collect::<Vec<_>>()
                );
                println!("Symbols: {:?}", symbols);
                println!("Part numbers: {:?}", part_numbers);
                println!("-------------------");

                row.push(node);
            }
            nodes.push(row);
        }

        Ok(Self {
            part_numbers,
            nodes,
        })
    }
}

impl Display for EngineSchematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.nodes {
            for node in row {
                write!(f, "{}", node)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn compute_back_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
        neighbors.push((x + 1, y - 1));
    }
    if x > 0 && y > 0 {
        neighbors.push((x - 1, y - 1));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_PATH: &str = "files/test.txt";

    #[test]
    fn test_part_numbers() {
        let schematic = EngineSchematic::try_from_file(INPUT_PATH).unwrap();
        eprintln!("Part numbers: {:?}", schematic.part_numbers);
        assert_eq!(schematic.part_numbers.len(), 8);
        assert_eq!(schematic.part_numbers.iter().sum::<u64>(), 4361);
    }
}
