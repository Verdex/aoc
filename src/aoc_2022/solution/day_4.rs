
use super::super::data::*;
use super::super::parsing::section::*;
use super::super::inputs::input::*;

fn contained(input : &(Section, Section)) -> bool {
    let (Section { start: s1, end: e1 }, Section { start: s2, end: e2 }) = input;

    (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1)
}

fn overlap(input : &(Section, Section)) -> bool {
    let (Section { start: s1, end: e1 }, Section { start: s2, end: e2 }) = input;

    (s1 >= s2 && s1 <= e2) || (e1 >= s2 && e1 <= e2)
    || (s2 >= s1 && s2 <= e1) || (e2 >= s1 && e2 <= e1)
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_4_1;
    let pairs = parse_section_pair_list(input);

    let result = pairs.into_iter().filter(contained).count();

    
    println!("2022 day 4:1 = {:?}", result);
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = DAY_4_1;
    let pairs = parse_section_pair_list(input);

    let result = pairs.into_iter().filter(overlap).count();

    
    println!("2022 day 4:1 = {:?}", result);
}