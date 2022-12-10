
use motif::*;

use super::super::data::*;

pub fn parse_asm(input : &str) -> Vec<Asm> {
    let mut i = input.char_indices();
    let output = asm(&mut i).unwrap();
    if i.count() != 0 { panic!("Did not consume all input"); }
    output
}

group!(asm: char => Vec<Asm> = |input| {
    seq!(noop: char => Asm = 'n', 'o', 'o', 'p', { Asm::Noop });
    seq!(addx_sym: char = 'a', 'd', 'd', 'x', { '\0' });
    seq!(addx: char => Asm = addx_sym, ! ws, neg <= ? minus, n <= ! number, {
        match neg {
            Some(_) => Asm::Addx(n * -1),
            None => Asm::Addx(n),
        }
    });
    alt!(instr: char => Asm = addx | noop);
    seq!(instr_end: char => Asm = i <= instr, end_line, { i });
    seq!(instrs: char => Vec<Asm> = xs <= * instr_end, mx <= ? instr, {
        let mut xs = xs;
        match mx {
            Some(x) => { xs.push(x); xs },
            None => xs,
        }
    });
    instrs(input)
});

pred!(minus: char = |x| x == '-');
pred!(ws: char = |x| x == ' ');

group!(end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(number: char => i64 = |input| {
    pred!(digit: char = |x| x.is_numeric());
    seq!(digits: char => i64 = d <= digit, ds <= * digit, { 
        let mut ds = ds;
        ds.insert(0, d);
        ds.iter().collect::<String>().parse::<i64>().expect("already parsed as valid number")
    });

    digits(input)
});