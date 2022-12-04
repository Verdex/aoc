
use motif::*;

use super::super::data::ElevatorInstruction;

pub fn parse_elevator_instructions( input : &str ) -> Vec<ElevatorInstruction> {
    let mut i = input.char_indices();
    let output = ei(&mut i).unwrap();
    if i.count() != 0 { panic!("all input not consumed"); }

    output
}

pub fn parse_dimensions( input : &str ) -> Vec<(u32, u32, u32)> {
    let mut i = input.char_indices();
    let output = dimensions(&mut i).unwrap();
    if i.count() != 0 { panic!("all input not consumed"); }
    output
}

group!(dimensions: char => Vec<(u32, u32, u32)> = |input| {
    pred!(x: char => () = |x| x == 'x' => { () });
    seq!(dim: char => (u32, u32, u32) = n1 <= number, x, n2 <= number, x, n3 <= number, {
        (n1, n2, n3)
    });

    seq!(dim_end: char => (u32, u32, u32) = d <= dim, end_line, { d });
    seq!(dims: char => Vec<(u32, u32, u32)> = ds <= * dim_end, md <= ? dim, {
        let mut ds = ds;
        match md {
            Some(d) => { ds.push(d); ds },
            None => ds
        }
    });

    dims(input)
});

group!(number: char => u32 = |input| {
    pred!(digit: char = |x| x.is_numeric());
    seq!(digits: char => u32 = d <= digit, ds <= * digit, { 
        let mut ds = ds;
        ds.insert(0, d);
        ds.iter().collect::<String>().parse::<u32>().expect("already parsed as valid number")
    });

    digits(input)
});

group!(end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(ei: char => Vec<ElevatorInstruction> = |input| {
    pred!(up: char => ElevatorInstruction = |x| x == '(' => { ElevatorInstruction::Up });
    pred!(down: char => ElevatorInstruction = |x| x == ')' => { ElevatorInstruction::Down });
    alt!(instr: char => ElevatorInstruction = up | down);
    seq!(main: char => Vec<ElevatorInstruction> = instrs <= * instr, { instrs });

    main(input)
});