mod engine_schematic;
mod node;
mod part_number;

use std::error::Error;

use engine_schematic::EngineSchematic;

static INPUT_PATH: &str = "files/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let schematic = EngineSchematic::try_from_file(INPUT_PATH)?;
    let result: u64 = schematic.part_numbers.iter().sum();
    println!("Result: {result}");
    Ok(())
}
