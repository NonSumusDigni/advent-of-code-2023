mod card;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::card::try_score_card;

// static INPUT_PATH: &str = "files/input.txt";
static INPUT_PATH: &str = "files/test.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(INPUT_PATH)?);

    let mut multipliers: HashMap<usize, BigUint> = HashMap::new();
    for line in reader.lines() {
        let (id, card_score) = try_score_card(&line?)?;
        let current_multiplier = multipliers.entry(id).or_insert(One::one()).to_owned();
        println!("Card {id} has {card_score} wins, with {current_multiplier} copies...");
        for i in (id + 1)..=(id + card_score) {
            let multiplier = multipliers.entry(i).or_insert(One::one()).clone();
            multipliers.insert(i, multiplier.add(&current_multiplier).clone());
        }
    }
    let final_score = multipliers
        .values()
        .fold(Zero::zero(), |acc: BigUint, x| acc.add(x));
    println!("Final score: {final_score}");
    Ok(())
}
