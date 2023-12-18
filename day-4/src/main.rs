mod card;

use crate::card::try_score_card;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

static INPUT_PATH: &str = "files/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(INPUT_PATH)?);

    let mut final_score = 0;
    for line in reader.lines() {
        final_score += try_score_card(&line?)?;
    }
    println!("Final score: {final_score}");
    Ok(())
}
