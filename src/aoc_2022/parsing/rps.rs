
use motif::*;

use super::super::data::*;

pub fn parse_rps_list(input : &str) -> Vec<(RPS, RPS)> {
    let mut i = input.char_indices();
    let output = rps_list(&mut i).unwrap();

    if i.count() != 0 { panic!("all input not consumed"); }

    output
}

pred!(ws: char => () = |x| x.is_whitespace() => { () });

group!(end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(rps_list: char => Vec<(RPS, RPS)> = |input| {
    pred!(first_rock: char => RPS = |x| x == 'A' => { RPS::Rock });
    pred!(first_paper: char => RPS = |x| x == 'B' => { RPS::Paper });
    pred!(first_scissors: char => RPS = |x| x == 'C' => { RPS::Scissors });
    
    pred!(second_rock: char => RPS = |x| x == 'X' => { RPS::Rock });
    pred!(second_paper: char => RPS = |x| x == 'Y' => { RPS::Paper });
    pred!(second_scissors: char => RPS = |x| x == 'Z' => { RPS::Scissors });

    alt!(first: char => RPS = first_rock | first_paper | first_scissors );
    alt!(second: char => RPS = second_rock | second_paper | second_scissors );

    seq!(row: char => (RPS, RPS) = f <= first, ws, s <= second, {
        (f, s)
    });

    seq!(row_end_line: char => (RPS, RPS) = r <= row, end_line, { r });

    seq!(rows: char => Vec<(RPS, RPS)> = rs <= * row_end_line, mr <= ? row, {
        let mut rs = rs;
        match mr {
            Some(r) => { rs.push(r); rs },
            None => rs,
        }
    });

    rows(input)
});
