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
    println!("Result: {:?}", result);
    Ok(())
}

fn get_input_reader<P>(input_file_path: P) -> Result<BufReader<File>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_file_path)?;
    Ok(BufReader::new(file))
}

#[allow(dead_code)]
#[derive(Debug)]
struct Output {
    pub part_1: u64,
    pub part_2: u64,
}

fn process_input(reader: BufReader<File>) -> Result<Output, Box<dyn Error>> {
    let query = GameAnalysis::new(12, 13, 14);
    let mut indexed_games = IndexedGames::default();

    let mut min_cube_sum = 0u64;
    for line in reader.lines() {
        let game = Game::try_from(line?.as_str())?;
        let analysis = game.analyze();
        min_cube_sum += analysis.min_cube();
        indexed_games.insert(game.id, analysis);
    }

    Ok(Output {
        part_1: indexed_games.query(query).iter().sum::<u64>(),
        part_2: min_cube_sum,
    })
}
