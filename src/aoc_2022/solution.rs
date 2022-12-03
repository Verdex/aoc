
use super::inputs::input::DAY_1_1;
use super::inputs::input::DAY_2_1;
use super::parsing;
use super::data::*;

pub fn solve_day_1_2() {

    let calories = parsing::calorie::parse_calorie_list_list(DAY_1_1);
    let mut results : Vec<u64> = calories.into_iter()
        .map(|x| x.into_iter().map(|y| y.0).sum())
        .collect::<Vec<_>>();
    
    results.sort();
    results.reverse();

    let result : u64 = results.into_iter().take(3).sum();
    println!("2022 day 1:2 = {}", result);
}

pub fn solve_day_2_1() {
    use RPS::*;

    fn selection_score(choice : &RPS) -> u64 {
        match choice {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn status_score(opponent : &RPS, you : &RPS) -> u64 {
        match (opponent, you) {
            (Rock, Paper) => 6,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,

            (Paper, Rock) => 0,
            (Scissors, Paper) => 0,
            (Rock, Scissors) => 0,

            _ => 3,
        }
    }

    let guide = parsing::rps::parse_rps_list(DAY_2_1);
    let result : u64 = guide.into_iter()
                      .map(|(opponent, you)| selection_score(&you) + status_score(&opponent, &you))
                      .sum();

    println!("2022 day 2:1 = {}", result);
}