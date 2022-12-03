
use super::inputs::input::DAY_1_1;
use super::parsing;

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
