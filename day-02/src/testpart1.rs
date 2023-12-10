#[cfg(test)]
mod tests {
    use crate::game::{parse_game_list, Game, Round};

    const SAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn parsing() {
        let game = parse_game_list(SAMPLE);
        let data = vec![
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        blue: 3,
                        red: 4,
                        green: 0,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        blue: 1,
                        green: 2,
                        red: 0,
                    },
                    Round {
                        green: 3,
                        blue: 4,
                        red: 1,
                    },
                    Round {
                        green: 1,
                        blue: 1,
                        red: 0,
                    },
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    Round {
                        green: 8,
                        blue: 6,
                        red: 20,
                    },
                    Round {
                        blue: 5,
                        red: 4,
                        green: 13,
                    },
                    Round {
                        green: 5,
                        red: 1,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    Round {
                        green: 1,
                        red: 3,
                        blue: 6,
                    },
                    Round {
                        green: 3,
                        red: 6,
                        blue: 0,
                    },
                    Round {
                        green: 3,
                        blue: 15,
                        red: 14,
                    },
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    Round {
                        red: 6,
                        blue: 1,
                        green: 3,
                    },
                    Round {
                        blue: 2,
                        red: 1,
                        green: 2,
                    },
                ],
            },
        ];

        assert_eq!(game, data);
    }

    #[test]
    fn summing() {
        let game = parse_game_list(SAMPLE);
        let sum: u32 = game
            .into_iter()
            .filter(Game::is_possible)
            .map(|game| game.id)
            .sum();

        assert_eq!(sum, 8);
    }
}
