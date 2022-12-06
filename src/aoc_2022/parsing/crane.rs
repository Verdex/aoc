
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

    Err(MatchError::Fatal(0))
});
