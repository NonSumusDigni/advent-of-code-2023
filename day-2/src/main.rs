mod game;
mod index;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use game::{Game, GameAnalysis};
use index::IndexedGames;

static INPUT_PATH: &str = "files/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let reader = get_input_reader(INPUT_PATH)?;
    let result = process_input(reader)?;
    println!("Result: {}", result);
    Ok(())
}

fn get_input_reader<P>(input_file_path: P) -> Result<BufReader<File>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_file_path)?;
    Ok(BufReader::new(file))
}

fn process_input(reader: BufReader<File>) -> Result<u64, Box<dyn Error>> {
    let query = GameAnalysis::new(12, 13, 14);
    let mut indexed_games = IndexedGames::default();

    for line in reader.lines() {
        indexed_games.insert(Game::try_from(line?.as_str())?);
    }

    Ok(indexed_games.query(query).iter().sum::<u64>())
}
