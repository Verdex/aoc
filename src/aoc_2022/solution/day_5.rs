
use super::super::parsing::crane::*;
use super::super::inputs::input::*;

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_5_1;
    let mut scenario = parse_crane(input);

    for instr in scenario.instrs {
        let src = scenario.stacks.get_mut(&instr.src).unwrap();

        let mut t = vec![];
        for _ in 1..=instr.count {
            t.push(src.pop().unwrap());
        }

        let dest = scenario.stacks.get_mut(&instr.dest).unwrap();

        for x in t {
            dest.push(x);
        }
    }

    let mut result = vec![];
    for index in 1..=9 {
        let v = scenario.stacks.get(&index).unwrap();
        let c = v.last().unwrap();
        result.push(c.clone());
    }

    println!("2022 day 5:1 = {}", result.into_iter().collect::<String>());
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = DAY_5_1;
    let mut scenario = parse_crane(input);

    for instr in scenario.instrs {
        let src = scenario.stacks.get_mut(&instr.src).unwrap();

        let mut t = vec![];
        for _ in 1..=instr.count {
            t.push(src.pop().unwrap());
        }
        t.reverse();

        let dest = scenario.stacks.get_mut(&instr.dest).unwrap();

        for x in t {
            dest.push(x);
        }
    }

    let mut result = vec![];
    for index in 1..=9 {
        let v = scenario.stacks.get(&index).unwrap();
        let c = v.last().unwrap();
        result.push(c.clone());
    }

    println!("2022 day 5:1 = {}", result.into_iter().collect::<String>());
}