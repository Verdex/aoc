
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Calorie(pub u64);

#[derive(Debug, Clone, Copy)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Status {
    Win,
    Draw,
    Lose
}