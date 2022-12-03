
use super::super::inputs::input::*;
use super::super::parsing;
use super::super::data::*;

use Status::*;
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

fn status_to_play(opponent : &RPS, goal : &Status) -> RPS {
    match (opponent, goal) {
        (o, Draw) => *o,

        (Rock, Lose) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Lose) => Paper,

        (Rock, Win) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Win) => Rock,
    }
}

pub fn solve_1() {

    let guide = parsing::rps::parse_rps_list(DAY_2_1);
    let result : u64 = guide.into_iter()
                      .map(|(opponent, you)| selection_score(&you) + status_score(&opponent, &you))
                      .sum();

    println!("2022 day 2:1 = {}", result);
}

pub fn solve_2() {

    let guide = parsing::rps::parse_rps_list_correctly(DAY_2_1);
    let result : u64 = guide.into_iter()
                      .map(|(opponent, goal)| {
                        let you = status_to_play(&opponent, &goal);
                        selection_score(&you) + status_score(&opponent, &you)
                      })
                      .sum();

    println!("2022 day 2:2 = {}", result);
}
