
use super::super::data::*;
use super::super::inputs::input::*;
use super::super::parsing::asm::*;

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_10_1;
    let asm = parse_asm(input);

    let targets = [20, 60, 100, 140, 180, 220];

    let mut cycle = 1;
    let mut x = 1;

    let states = asm.into_iter().flat_map(move |instr| {
        match instr {
            Asm::Addx(v) => { 
                let ret = vec![(cycle + 1, x), (cycle + 2, x + v)];
                x += v; 
                cycle += 2; 
                ret
            },
            Asm::Noop => { cycle += 1; vec![(cycle, x)] },
        }
    });

    let target_states = states.filter(|(cycle, _)| targets.iter().any(|x| x == cycle));

    let result : i64 = target_states.map(|(a,b)| a * b).sum();

    println!("2022 day 10:1 = {}", result);
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = DAY_10_1;
    let asm = parse_asm(input);

    let mut cycle = 0;
    let mut x = 1;

    let states = asm.into_iter().flat_map(move |instr| {
        match instr {
            Asm::Addx(v) => { 
                let ret = vec![(cycle + 1, x), (cycle + 2, x)];
                x += v; 
                cycle += 2; 
                ret
            },
            Asm::Noop => { cycle += 1; vec![(cycle, x)] },
        }
    });

    let sprite_positions = states.map(|(cycle, pos)| ((cycle - 1) % 40, vec![pos -1, pos, pos +1])).collect::<Vec<_>>();
    let pos_lit = sprite_positions.into_iter().map(|(pos, sprite)| sprite.iter().any(|s| *s == pos))
        .map(|b| if b { '#' } else { '.' }).collect::<Vec<_>>();

    let output = [ &pos_lit[0..40]
                 , &pos_lit[40..80]
                 , &pos_lit[80..120]
                 , &pos_lit[120..160]
                 , &pos_lit[160..200]
                 , &pos_lit[200..240]
                 ];

    println!("2022 day 10:2");

    for line in output {
        println!("{}", line.iter().collect::<String>());
    }
}
