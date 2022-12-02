
use motif::*;

use super::super::data::ElevatorInstruction;

pub fn parse_elevator_instructions( input : &str ) -> Vec<ElevatorInstruction> {
    let mut i = input.char_indices();
    let output = ei(&mut i).unwrap();
    if i.count() != 0 { panic!("all input not consumed"); }

    output
}

group!(ei: char => Vec<ElevatorInstruction> = |input| {
    pred!(up: char => ElevatorInstruction = |x| x == '(' => { ElevatorInstruction::Up });
    pred!(down: char => ElevatorInstruction = |x| x == ')' => { ElevatorInstruction::Down });
    alt!(instr: char => ElevatorInstruction = up | down);
    seq!(main: char => Vec<ElevatorInstruction> = instrs <= * instr, { instrs });

    main(input)
});