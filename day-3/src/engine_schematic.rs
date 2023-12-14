use crate::gear::{GearBuilder, GearState};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use crate::node::Node;
use crate::part_number::{PartNumberBuilder, PartNumbers};

pub struct EngineSchematic {
    pub part_numbers: Vec<u64>,
    pub gear_ratios: Vec<u64>,
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

        let mut unvalidated_part_numbers = PartNumbers::new();
        let mut unvalidated_gears: HashMap<(usize, usize), Rc<RefCell<GearBuilder>>> =
            HashMap::new();
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
                            let mut saw_symbol = false;
                            for &(neighbor_x, neighbor_y) in compute_back_neighbors(x, y).iter() {
                                if !symbols.contains(&(neighbor_x, neighbor_y)) {
                                    continue;
                                }

                                saw_symbol = true;
                                part_number_builder.borrow_mut().validate();
                                if unvalidated_gears.contains_key(&(neighbor_x, neighbor_y)) {
                                    part_number_builder.borrow_mut().add_gear(
                                        unvalidated_gears
                                            .get(&(neighbor_x, neighbor_y))
                                            .expect("Unreachable: No gear")
                                            .clone(),
                                        neighbor_x,
                                        neighbor_y,
                                    );
                                }
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
                            }

                            if !saw_symbol {
                                unvalidated_part_numbers
                                    .insert((x, y), part_number_builder.clone());
                            }
                        }
                    }
                    Node::Symbol(character) => {
                        symbols.insert((x, y));
                        let is_gear = character == '*';
                        if is_gear {
                            let gear_builder = Rc::new(RefCell::new(GearBuilder::new()));
                            unvalidated_gears.insert((x, y), gear_builder.clone());
                        }
                        let mut seen_part_numbers = HashSet::new();
                        compute_back_neighbors(x, y).iter().for_each(|n| {
                            if let Some(part_number_builder) =
                                unvalidated_part_numbers.get(n).cloned()
                            {
                                if is_gear
                                    && !seen_part_numbers
                                        .contains(&part_number_builder.borrow().key())
                                {
                                    seen_part_numbers.insert(part_number_builder.borrow().key());
                                    part_number_builder.borrow_mut().add_gear(
                                        unvalidated_gears
                                            .get(&(x, y))
                                            .expect("Unreachable: No gear")
                                            .clone(),
                                        x,
                                        y,
                                    );
                                }

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

                // TODO this sucks
                let mut gears_to_remove = vec![];
                for (coords, gear_builder) in unvalidated_gears.iter() {
                    if gear_builder.borrow().get_state() == GearState::Seized {
                        gears_to_remove.push(*coords);
                    }
                }
                for coords in gears_to_remove {
                    unvalidated_gears.remove(&coords);
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
                println!(
                    "Unvalidated gears: {:?}",
                    unvalidated_gears
                        .iter()
                        .map(|(k, v)| (*k, v.borrow().get_state()))
                        .collect::<Vec<_>>()
                );
                println!("Symbols: {:?}", symbols);
                println!("Part numbers: {:?}", part_numbers);
                println!("-------------------");

                row.push(node);
            }

            if let Some(part_number_builder) = current_part_number_builder {
                part_number_builder.borrow_mut().complete();
                if part_number_builder.borrow().buildable() {
                    let built = part_number_builder.borrow().build();
                    println!("Built: {}, code loc: row-end", built);
                    part_numbers.push(built);
                    part_number_builder.borrow().nodes().iter().for_each(|n| {
                        unvalidated_part_numbers.remove(n);
                    });
                }
                current_part_number_builder = None;
            }

            nodes.push(row);
        }

        let gear_ratios = unvalidated_gears
            .values()
            .filter(|gear_builder| gear_builder.borrow().get_state() == GearState::Buildable)
            .map(|gear_builder| gear_builder.borrow().build())
            .collect::<Vec<_>>();

        Ok(Self {
            part_numbers,
            gear_ratios,
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
    fn test_build() {
        let schematic = EngineSchematic::try_from_file(INPUT_PATH).unwrap();
        assert_eq!(schematic.part_numbers.len(), 9);
        assert_eq!(schematic.part_numbers.iter().sum::<u64>(), 7475);
        assert_eq!(schematic.gear_ratios.len(), 1);
        assert_eq!(schematic.gear_ratios.iter().sum::<u64>(), 451_490);
    }
}
