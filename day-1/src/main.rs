use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

static INPUT_FILE_PATH: &str = "inputs/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let file_reader = get_input_reader(INPUT_FILE_PATH)?;
    let result = process_input(file_reader)?;
    println!("Result: {result}");
    Ok(())
}

fn get_input_reader<P>(input_file_path: P) -> Result<BufReader<File>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_file_path)?;
    Ok(BufReader::new(file))
}

fn process_input(reader: BufReader<File>) -> Result<u32, Box<dyn Error>> {
    Ok(reader
        .lines()
        .map(|line| parse_u32(line?))
        .collect::<Result<Vec<u32>, Box<dyn Error>>>()?
        .into_iter()
        .fold(0, |acc, num| acc + num))
}

fn parse_u32(line: String) -> Result<u32, Box<dyn Error>> {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for c in line.chars() {
        if c.is_ascii_digit() {
            if first.is_none() {
                first = Some(c);
            } else {
                last = Some(c);
            }
        }
    }
    let digits = match (first, last) {
        (Some(f), Some(l)) => vec![f, l],
        (Some(f), None) => vec![f, f],
        _ => panic!("invalid digits: {}", line),
    };
    Ok(digits.into_iter().collect::<String>().parse()?)
}
