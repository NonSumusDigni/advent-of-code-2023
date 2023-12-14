mod engine_schematic;
mod gear;
mod node;
mod part_number;

use std::error::Error;

use engine_schematic::EngineSchematic;

static INPUT_PATH: &str = "files/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let schematic = EngineSchematic::try_from_file(INPUT_PATH)?;
    let result_1: u64 = schematic.part_numbers.iter().sum();
    println!("Result1: {result_1}");
    let result_2: u64 = schematic.gear_ratios.iter().sum();
    println!("Result2: {result_2}");
    Ok(())
}
