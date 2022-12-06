
use motif::*;

use super::super::data::*;

pub fn parse_crane( input : &str ) -> CraneScenario {
    let mut i = input.char_indices();
    let output = scenario(&mut i).unwrap();
    if i.count() != 0 { panic!("not all input is consumed"); }
    output
}

pred!(ws: char = |x| x == ' ');

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

group!(scenario_row: char => Vec<Option<char>> = |input| {
    pred!(lsquare: char = |x| x == '[');
    pred!(rsquare: char = |x| x == ']');
    pred!(letter: char = |x| x.is_alphabetic());
    seq!(container: char => Option<char> = lsquare, l <= letter, rsquare, { Some(l) });
    seq!(empty: char => Option<char> = ws, ws, ws, { None });
    alt!(item: char => Option<char> = empty | container);
    seq!(item_ws: char => Option<char> = x <= item, ws, { x });

    seq!(main: char => Vec<Option<char>> = xs <= * item_ws, x <= item, end_line, { 
        let mut xs = xs;
        xs.push(x);
        xs
    });

    main(input)
});

group!(scenario: char => CraneScenario = |input| {

    seq!(main: char => CraneScenario 
        = rows <= * scenario_row
        , instructions <= instrs 
        , {
            use std::collections::HashMap;

            let mut stacks : HashMap<usize, Vec<char>> = (1..=9).into_iter().map(|x| (x, vec![])).collect();

            for x in 0..=8 {
                for row in rows.iter() {
                    match row[x] {
                        Some(item) => { 
                            let target = stacks.get_mut(&(x + 1)).unwrap(); 
                            target.push(item);
                        },
                        None => { },
                    }
                }
            }

            for x in 1..=9 {
                let target = stacks.get_mut(&x).unwrap();
                target.reverse();
            }

            CraneScenario { instrs: instructions, stacks }
        });

    main(input)
});

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn scenario_row_should_parse() {
        let input = "    [B]             [B] [S]        

";

        let mut i = input.char_indices();
        let output = scenario_row(&mut i).unwrap();
        assert_eq!( output.len(), 9 );
        assert_eq!( output[0], None );
        assert_eq!( output[1], Some('B') );
        assert_eq!( output[2], None );
        assert_eq!( output[3], None );
        assert_eq!( output[4], None );
        assert_eq!( output[5], Some('B') );
        assert_eq!( output[6], Some('S') );
        assert_eq!( output[7], None );
        assert_eq!( output[8], None );
    }

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
