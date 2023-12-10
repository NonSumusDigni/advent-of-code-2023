use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum Error {
    /// Invalid game string: {0}
    InvalidGameString(String),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Game {
    pub(crate) id: u64,
    iterations: Vec<GameIteration>,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct GameIteration {
    red: u64,
    blue: u64,
    green: u64,
}

impl Game {
    pub fn analyze(&self) -> GameAnalysis {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for iteration in &self.iterations {
            if iteration.red > max_red {
                max_red = iteration.red;
            }
            if iteration.blue > max_blue {
                max_blue = iteration.blue;
            }
            if iteration.green > max_green {
                max_green = iteration.green;
            }
        }

        GameAnalysis {
            max_red,
            max_blue,
            max_green,
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut game = Game::default();

        let (game_prefix, game_body) = value
            .split_once(": ")
            .ok_or_else(|| Error::InvalidGameString(format!("invalid game string: {}", value)))?;

        let id = game_prefix
            .strip_prefix("Game ")
            .ok_or_else(|| Error::InvalidGameString(format!("invalid game string: {}", value)))?
            .parse::<u64>()
            .map_err(|_| Error::InvalidGameString(format!("invalid game string: {}", value)))?;

        game.id = id;

        for game_iteration_body in game_body.split("; ") {
            let mut game_iteration = GameIteration::default();

            for color_result in game_iteration_body.split(", ") {
                let (color_count_str, color_body) =
                    color_result.split_once(" ").ok_or_else(|| {
                        Error::InvalidGameString(format!("invalid game string: {}", value))
                    })?;

                let color_count = color_count_str.parse::<u64>().map_err(|_| {
                    Error::InvalidGameString(format!("invalid game string: {}", value))
                })?;

                match color_body {
                    "red" => game_iteration.red = color_count,
                    "blue" => game_iteration.blue = color_count,
                    "green" => game_iteration.green = color_count,
                    _ => {
                        return Err(Error::InvalidGameString(format!(
                            "invalid game string: {}",
                            value
                        )))
                    }
                }
            }

            game.iterations.push(game_iteration);
        }

        Ok(game)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct GameAnalysis {
    pub max_red: u64,
    pub max_blue: u64,
    pub max_green: u64,
}

impl GameAnalysis {
    pub fn new(red: u64, green: u64, blue: u64) -> Self {
        Self {
            max_red: red,
            max_blue: blue,
            max_green: green,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r#"Game 1: 1 red, 1 blue, 1 green; 2 red, 2 blue, 2 green; 3 red, 3 blue, 3 green"#,
        Game {
            id: 1,
            iterations: vec![
                GameIteration {
                    red: 1,
                    blue: 1,
                    green: 1,
                },
                GameIteration {
                    red: 2,
                    blue: 2,
                    green: 2,
                },
                GameIteration {
                    red: 3,
                    blue: 3,
                    green: 3,
                },
            ],
        }
        ; "game 1"
    )]
    #[test_case(
        r#"Game 2: 3 green, 4 red, 4 blue; 6 red, 4 green, 4 blue; 2 blue, 4 green, 3 red"#,
        Game {
            id: 2,
            iterations: vec![
                GameIteration {
                    red: 4,
                    blue: 4,
                    green: 3,
                },
                GameIteration {
                    red: 6,
                    blue: 4,
                    green: 4,
                },
                GameIteration {
                    red: 3,
                    blue: 2,
                    green: 4,
                },
            ],
        }
        ; "game 2"
    )]
    #[test_case(
        r#"Game 3: 1 red, 2 green, 3 blue; 1 red, 2 green; 2 green, 3 red; 1 blue, 2 red"#,
        Game {
            id: 3,
            iterations: vec![
                GameIteration {
                    red: 1,
                    blue: 3,
                    green: 2,
                },
                GameIteration {
                    red: 1,
                    blue: 0,
                    green: 2,
                },
                GameIteration {
                    red: 3,
                    blue: 0,
                    green: 2,
                },
                GameIteration {
                    red: 2,
                    blue: 1,
                    green: 0,
                },
            ],
        }
        ; "game 3"
    )]
    #[test_case(
        r#"Game 4: 1 red, 15 green; 1 green, 2 blue; 12 green, 1 red, 2 blue; 14 green; 2 green, 1 blue, 2 red"#,
        Game {
            id: 4,
            iterations: vec![
                GameIteration {
                    red: 1,
                    blue: 0,
                    green: 15,
                },
                GameIteration {
                    red: 0,
                    blue: 2,
                    green: 1,
                },
                GameIteration {
                    red: 1,
                    blue: 2,
                    green: 12,
                },
                GameIteration {
                    red: 0,
                    blue: 0,
                    green: 14,
                },
                GameIteration {
                    red: 2,
                    blue: 1,
                    green: 2,
                },
            ],
        }
        ; "game 4"
    )]
    #[test_case(
        r#"Game 5: 8 red; 7 red; 11 red, 4 green; 1 blue, 8 red; 6 red, 2 green, 1 blue; 8 green, 13 red, 1 blue"#,
        Game {
            id: 5,
            iterations: vec![
                GameIteration {
                    red: 8,
                    blue: 0,
                    green: 0,
                },
                GameIteration {
                    red: 7,
                    blue: 0,
                    green: 0,
                },
                GameIteration {
                    red: 11,
                    blue: 0,
                    green: 4,
                },
                GameIteration {
                    red: 8,
                    blue: 1,
                    green: 0,
                },
                GameIteration {
                    red: 6,
                    blue: 1,
                    green: 2,
                },
                GameIteration {
                    red: 13,
                    blue: 1,
                    green: 8,
                },
            ],
        }
        ; "game 5"
    )]
    #[test_case(
        r#"Game 6: 3 blue, 2 red, 6 green; 2 red, 8 green, 1 blue; 1 red, 3 blue"#,
        Game {
            id: 6,
            iterations: vec![
                GameIteration {
                    red: 2,
                    blue: 3,
                    green: 6,
                },
                GameIteration {
                    red: 2,
                    blue: 1,
                    green: 8,
                },
                GameIteration {
                    red: 1,
                    blue: 3,
                    green: 0,
                },
            ],
        }
        ; "game 6"
    )]
    #[test_case(
        r#"Game 7: 5 green, 1 red, 1 blue; 6 blue, 12 red; 6 red, 7 green; 3 green, 1 blue"#,
        Game {
            id: 7,
            iterations: vec![
                GameIteration {
                    red: 1,
                    blue: 1,
                    green: 5,
                },
                GameIteration {
                    red: 12,
                    blue: 6,
                    green: 0,
                },
                GameIteration {
                    red: 6,
                    blue: 0,
                    green: 7,
                },
                GameIteration {
                    red: 0,
                    blue: 1,
                    green: 3,
                },
            ],
        }
        ; "game 7"
    )]
    #[test_case(
        r#"Game 8: 10 red, 6 green; 4 blue, 6 green, 5 red; 8 green, 5 red, 5 blue; 2 red, 4 blue"#,
        Game {
            id: 8,
            iterations: vec![
                GameIteration {
                    red: 10,
                    blue: 0,
                    green: 6,
                },
                GameIteration {
                    red: 5,
                    blue: 4,
                    green: 6,
                },
                GameIteration {
                    red: 5,
                    blue: 5,
                    green: 8,
                },
                GameIteration {
                    red: 2,
                    blue: 4,
                    green: 0,
                },
            ],
        }
        ; "game 8"
    )]
    #[test_case(
        r#"Game 9: 11 blue, 13 red, 3 green; 13 red, 1 green, 6 blue; 8 blue, 4 green, 5 red; 16 red, 7 green, 10 blue; 16 red, 5 green, 6 blue; 17 red, 6 blue"#,
        Game {
            id: 9,
            iterations: vec![
                GameIteration {
                    red: 13,
                    blue: 11,
                    green: 3,
                },
                GameIteration {
                    red: 13,
                    blue: 6,
                    green: 1,
                },
                GameIteration {
                    red: 5,
                    blue: 8,
                    green: 4,
                },
                GameIteration {
                    red: 16,
                    blue: 10,
                    green: 7,
                },
                GameIteration {
                    red: 16,
                    blue: 6,
                    green: 5,
                },
                GameIteration {
                    red: 17,
                    blue: 6,
                    green: 0,
                },
            ],
        }
        ; "game 9"
    )]
    #[test_case(
        r#"Game 10: 16 blue, 8 green; 2 red, 4 green, 1 blue; 15 blue; 4 red, 5 green, 4 blue"#,
        Game {
            id: 10,
            iterations: vec![
                GameIteration {
                    red: 0,
                    blue: 16,
                    green: 8,
                },
                GameIteration {
                    red: 2,
                    blue: 1,
                    green: 4,
                },
                GameIteration {
                    red: 0,
                    blue: 15,
                    green: 0,
                },
                GameIteration {
                    red: 4,
                    blue: 4,
                    green: 5,
                },
            ],
        }
        ; "game 10"
    )]
    fn test_game_deserialize(input: &str, expected: Game) {
        let result = Game::try_from(input).expect("deserialize");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_analysis_from_game() {
        let game = Game {
            id: 1,
            iterations: vec![
                GameIteration {
                    red: 10,
                    blue: 1,
                    green: 1,
                },
                GameIteration {
                    red: 2,
                    blue: 0,
                    green: 22,
                },
                GameIteration {
                    red: 2,
                    blue: 0,
                    green: 0,
                },
            ],
        };

        let result = game.analyze();

        assert_eq!(
            result,
            GameAnalysis {
                max_red: 10,
                max_blue: 1,
                max_green: 22,
            }
        );
    }
}
