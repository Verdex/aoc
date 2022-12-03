
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Calorie(pub u64);

#[derive(Debug)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}