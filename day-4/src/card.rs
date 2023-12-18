use std::collections::HashSet;
use std::error::Error;

pub fn try_score_card(input: &str) -> Result<usize, Box<dyn Error>> {
    let (_label, info) = input.split_once(':').ok_or("Invalid input")?;
    // let (_, id) = label.split_once(' ').ok_or("Invalid input")?;
    let (winners, picks) = info.trim().split_once(" | ").ok_or("Invalid input")?;
    let winners = winners
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>())
        .collect::<Result<HashSet<usize>, _>>()?;
    let picks = picks
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>());

    let mut score = None::<usize>;
    for pick in picks {
        let pick = pick?;
        if winners.contains(&pick) {
            score = match score {
                None => Some(1),
                Some(s) => Some(s * 2),
            }
        }
    }

    Ok(score.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_score_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = try_score_card(input).unwrap();
        assert_eq!(result, 8);
    }
}
