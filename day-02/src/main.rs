mod game;
mod input;
mod testpart1;
mod testpart2;

use game::parse_game_list;

fn main() {
    let game = parse_game_list(input::INPUT);
    let sum: u32 = game
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum();

    let min_dice: u32 = game
        .iter()
        .map(|game| game.min_dice())
        .map(|round| round.red * round.green * round.blue)
        .sum();

    println!("The first sum is  {}", sum);
    println!("The second sum is {}", min_dice);
}
