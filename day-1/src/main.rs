use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use lazy_static::lazy_static;
use trie_rs::{Trie, TrieBuilder};

static INPUT_FILE_PATH: &str = "inputs/input.txt";

lazy_static! {
    static ref DIGIT_TRIE: Trie<u8> = {
        let mut builder = TrieBuilder::new();
        builder.push("zero");
        builder.push("one");
        builder.push("two");
        builder.push("three");
        builder.push("four");
        builder.push("five");
        builder.push("six");
        builder.push("seven");
        builder.push("eight");
        builder.push("nine");
        builder.push("0");
        builder.push("1");
        builder.push("2");
        builder.push("3");
        builder.push("4");
        builder.push("5");
        builder.push("6");
        builder.push("7");
        builder.push("8");
        builder.push("9");
        builder.build()
    };
}

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

fn process_input(reader: BufReader<File>) -> Result<u64, Box<dyn Error>> {
    Ok(reader
        .lines()
        .map(|line| parse_u8(line?))
        .collect::<Result<Vec<u8>, Box<dyn Error>>>()?
        .into_iter()
        .fold(0u64, |acc, num| acc + num as u64))
}

#[derive(Debug)]
struct DigitBuffer {
    state: DigitState,
    start_index: usize,
    end_index: usize,
}

impl DigitBuffer {
    pub fn new(index: usize) -> Self {
        Self {
            state: DigitState::Uninitialized,
            start_index: index,
            end_index: index,
        }
    }

    pub fn advance_index(&mut self, index: usize, string: &str) -> DigitState {
        self.end_index = index + 1;
        let slice = &string[self.start_index..self.end_index];

        self.state = if DIGIT_TRIE.exact_match(slice) {
            DigitState::Match(normalize_digit(slice))
        } else if DIGIT_TRIE.predictive_search(slice).is_empty() {
            DigitState::Invalid
        } else {
            DigitState::Partial
        };

        self.state
    }
}

#[derive(Clone, Copy, Debug)]
enum DigitState {
    Uninitialized,
    Partial,
    Invalid,
    Match(char),
}

impl DigitState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Invalid | Self::Match(_))
    }
}

fn normalize_digit(string: &str) -> char {
    match string {
        "zero" | "0"    => '0',
        "one" | "1"     => '1',
        "two" | "2"     => '2',
        "three" | "3"   => '3',
        "four" | "4"    => '4',
        "five" | "5"    => '5',
        "six" | "6"     => '6',
        "seven" | "7"   => '7',
        "eight" | "8"   => '8',
        "nine" | "9"    => '9',
        _ => panic!("normalize_digit")
    }
}

fn parse_u8(line: String) -> Result<u8, Box<dyn Error>> {
    let mut digit_buffers: HashMap<usize, DigitBuffer> = HashMap::new();
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    let mut index = 0;
    while index < line.len() {
        let new_buffer = DigitBuffer::new(index);
        digit_buffers.insert(index, new_buffer);

        let mut digits_to_remove = vec![];
        for digit_buffer in digit_buffers.values_mut() {
            let new_state = digit_buffer.advance_index(index, &line);

            if let DigitState::Match(digit) = new_state {
                if first.is_none() {
                    first = Some(digit);
                }
                last = Some(digit);
            }

            if new_state.is_terminal() {
                digits_to_remove.push(digit_buffer.start_index);
            }
        }
        for digit in digits_to_remove {
            digit_buffers.remove(&digit);
        }

        index += 1;
    }

    let digits = match (first, last) {
        (Some(f), Some(l)) => vec![f, l],
        _ => panic!("invalid digits: {}", line),
    };

    Ok(digits.into_iter().collect::<String>().parse()?)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine",         29)]
    #[test_case("eightwothree",     83)]
    #[test_case("abcone2threexyz",  13)]
    #[test_case("xtwone3four",      24)]
    #[test_case("4nineeightseven2", 42)]
    #[test_case("zoneight234",      14)]
    #[test_case("7pqrstsixteen",    76)]
    fn test_parse_u8(line: &str, expected: u8) {
        let result = parse_u8(line.to_string()).expect("ok");
        assert_eq!(result, expected);
    }
}