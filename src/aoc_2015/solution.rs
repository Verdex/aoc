

use super::data::*;
use super::parsing::parser::*;
use super::inputs::input::*;

pub fn solve_day_1_1() {
    let result : i32 = parse_elevator_instructions(DAY_1_1)
        .into_iter()
        .map(|x| match x { ElevatorInstruction::Up => 1, ElevatorInstruction::Down => -1 })
        .sum();

    println!("day 1:1 = {}", result);
}

pub fn solve_day_1_2() {
    let numbers = parse_elevator_instructions(DAY_1_1)
        .into_iter()
        .map(|x| match x { ElevatorInstruction::Up => 1, ElevatorInstruction::Down => -1 })
        .collect::<Vec<i32>>();

    let mut cur = 0;
    for (pos, n) in numbers.into_iter().enumerate() { 
        cur += n;
        if cur <= -1 {
            println!("day 1:2 = {}", pos + 1);
            break;
        }
    }
}