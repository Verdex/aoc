

use super::data::*;
use super::parsing::parser::*;
use super::inputs::input::*;

#[allow(dead_code)]
pub fn solve_day_1_1() {
    let result : i32 = parse_elevator_instructions(DAY_1_1)
        .into_iter()
        .map(|x| match x { ElevatorInstruction::Up => 1, ElevatorInstruction::Down => -1 })
        .sum();

    println!("2015 day 1:1 = {}", result);
}

#[allow(dead_code)]
pub fn solve_day_1_2() {
    let numbers = parse_elevator_instructions(DAY_1_1)
        .into_iter()
        .map(|x| match x { ElevatorInstruction::Up => 1, ElevatorInstruction::Down => -1 })
        .collect::<Vec<i32>>();

    let mut cur = 0;
    for (pos, n) in numbers.into_iter().enumerate() { 
        cur += n;
        if cur <= -1 {
            println!("2015 day 1:2 = {}", pos + 1);
            break;
        }
    }
}

#[allow(dead_code)]
pub fn solve_day_2_1() { 
    fn req_paper(input : (u32, u32, u32)) -> u32 {
        let l = input.0;
        let w = input.1;
        let h = input.2;

        let mut x = vec![l*w, w*h, h*l];
        x.sort();
        let extra = x[0];
        x.into_iter().map(|y| y * 2).sum::<u32>() + extra
    }

    let dims = parse_dimensions(DAY_2_1);

    let result = dims.into_iter().map(|x| req_paper(x)).sum::<u32>();

    println!("2015 day 2:1 = {}", result);
}

#[allow(dead_code)]
pub fn solve_day_2_2() { 
    fn req_bow(input : (u32, u32, u32)) -> u32 {
        let l = input.0;
        let w = input.1;
        let h = input.2;

        let mut x = vec![l, w, h];
        x.sort();
        let a = x[0] * 2;
        let b = x[1] * 2;
        a + b + (l * w * h)
    }

    let dims = parse_dimensions(DAY_2_1);

    let result = dims.into_iter().map(|x| req_bow(x)).sum::<u32>();

    println!("2015 day 2:2 = {}", result);
}

#[allow(dead_code)]
pub fn solve_day_3_1() {
    let input = DAY_3_1.chars();

    let mut x = 0;
    let mut y = 0;

    let mut v = vec![(0, 0)];

    for dir in input {
        match dir {
            '^' => { y+=1; v.push((x,y)); },
            '<' => { x-=1; v.push((x,y)); },
            '>' => { x+=1; v.push((x,y)); },
            'v' => { y-=1; v.push((x,y)); },
            _ => unreachable!(),
        }
    }

    v.sort();
    v.dedup();

    println!("2015 day 3:1 = {}", v.len());
}

#[allow(dead_code)]
pub fn solve_day_3_2() {
    let input = DAY_3_1.chars();

    let mut x1 = 0;
    let mut y1 = 0;

    let mut x2 = 0;
    let mut y2 = 0;

    let mut v = vec![(0, 0)];

    let mut b = false;

    for dir in input {
        match (dir, b) {
            ('^', true) => { y1+=1; v.push((x1,y1)); },
            ('<', true) => { x1-=1; v.push((x1,y1)); },
            ('>', true) => { x1+=1; v.push((x1,y1)); },
            ('v', true) => { y1-=1; v.push((x1,y1)); },

            ('^', false) => { y2+=1; v.push((x2,y2)); },
            ('<', false) => { x2-=1; v.push((x2,y2)); },
            ('>', false) => { x2+=1; v.push((x2,y2)); },
            ('v', false) => { y2-=1; v.push((x2,y2)); },
            _ => unreachable!(),
        }
        b = !b;
    }

    v.sort();
    v.dedup();

    println!("2015 day 3:2 = {}", v.len());
}