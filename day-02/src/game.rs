pub fn parse_game_list(game_list: &str) -> Vec<Game> {
    game_list.split("\n").map(Game::new).collect()
}

pub fn max(game_a: &Round, game_b: &Round) -> Round {
    Round {
        red: std::cmp::max(game_a.red, game_b.red),
        green: std::cmp::max(game_a.green, game_b.green),
        blue: std::cmp::max(game_a.blue, game_b.blue),
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn new(line: &str) -> Game {
        let mut game_split = line.split(":");
        let id: u32 = game_split
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let rounds: Vec<Round> = game_split
            .next()
            .unwrap()
            .split(";")
            .map(Round::new)
            .collect();

        Game { id, rounds }
    }

    pub fn is_possible(self: &Self) -> bool {
        let impossible = self
            .rounds
            .iter()
            .any(|round| round.red > 12 || round.green > 13 || round.blue > 14);
        !impossible
    }

    pub fn min_dice(self: &Self) -> Round {
        let mut min_dice = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for round in self.rounds.iter() {
            min_dice = max(round, &min_dice);
        }

        min_dice
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Round {
    pub fn new(description: &str) -> Round {
        let parts = description.split(",").map(|x| {
            let mut iter = x.trim().split(" ");

            let count = iter.next().unwrap().parse::<u32>().unwrap();
            let color = iter.next().unwrap();

            (count, color)
        });

        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in parts {
            let (count, color) = cube;

            match color {
                "red" => round.red = count,
                "green" => round.green = count,
                "blue" => round.blue = count,
                _ => panic!("Uh oh"),
            };
        }

        round
    }
}
