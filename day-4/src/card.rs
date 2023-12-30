use std::collections::HashSet;
use std::error::Error;

pub fn try_score_card(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (label, info) = input.split_once(':').ok_or("Invalid input")?;
    let (_, id) = label.split_once(' ').ok_or("Invalid input")?;
    let (winners, picks) = info.trim().split_once(" | ").ok_or("Invalid input")?;
    let winners = winners
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>())
        .collect::<Result<HashSet<usize>, _>>()?;
    let picks: Vec<usize> = picks
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    let score = picks.into_iter().filter(|n| winners.contains(n)).count();

    Ok((id.trim().parse()?, score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_score_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (id, score) = try_score_card(input).unwrap();
        assert_eq!(id, 1);
        assert_eq!(score, 4);
    }
}
