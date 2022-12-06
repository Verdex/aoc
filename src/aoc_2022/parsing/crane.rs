
use motif::*;

use super::super::data::*;

pub fn parse_crane( input : &str ) -> CraneScenario {

    panic!("!");
}

group!(end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(number: char => usize = |input| {
    pred!(digit: char = |x| x.is_numeric());
    seq!(digits: char => usize = d <= digit, ds <= * digit, { 
        let mut ds = ds;
        ds.insert(0, d);
        ds.iter().collect::<String>().parse::<usize>().expect("already parsed as valid number")
    });

    digits(input)
});

group!(instrs: char => Vec<CraneInstruction> = |input| {
    pred!(ws: char = |x| x.is_whitespace());
    seq!(move_sym: char => () = 'm', 'o', 'v', 'e', { () });
    seq!(from_sym: char => () = 'f', 'r', 'o', 'm', { () });
    seq!(to_sym: char => () = 't', 'o', { () });

    seq!(instr: char => CraneInstruction 
        = move_sym, ws, count <= number, ws, from_sym, ws, src <= number, ws, to_sym, ws, dest <= number
        , 
        {
            CraneInstruction { count, src, dest }
        });

    seq!(instr_end: char => CraneInstruction = instr <= instr, end_line, { instr });
    seq!(main: char => Vec<CraneInstruction> = xs <= * instr_end, mx <= ? instr, {
        let mut xs = xs;
        match mx { 
            Some(x) => { xs.push(x); xs },
            None => xs,
        }
    });

    main(input)
});

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn instrs_should_parse() {
        let input = "move 1 from 2 to 3
move 4 from 5 to 6";
        let mut i = input.char_indices();
        let output = instrs(&mut i).unwrap();
        assert_eq!( output.len(), 2 );
        assert_eq!( output[0].count, 1 );
        assert_eq!( output[0].src, 2 );
        assert_eq!( output[0].dest, 3 );
        assert_eq!( output[1].count, 4 );
        assert_eq!( output[1].src, 5 );
        assert_eq!( output[1].dest, 6 );
    }
}
