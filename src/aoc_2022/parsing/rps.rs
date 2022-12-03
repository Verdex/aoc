
use motif::*;

use super::super::data::*;

pub fn parse_rps_list(input : &str) -> Vec<(RPS, RPS)> {
    let mut i = input.char_indices();
    let output = rps_list(&mut i).unwrap();

    if i.count() != 0 { panic!("all input not consumed"); }

    output
}

pub fn parse_rps_list_correctly(input : &str) -> Vec<(RPS, Status)> {
    let mut i = input.char_indices();
    let output = correct_rps_list(&mut i).unwrap();

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

pred!(first_rock: char => RPS = |x| x == 'A' => { RPS::Rock });
pred!(first_paper: char => RPS = |x| x == 'B' => { RPS::Paper });
pred!(first_scissors: char => RPS = |x| x == 'C' => { RPS::Scissors });

alt!(first: char => RPS = first_rock | first_paper | first_scissors );

pred!(second_rock: char => RPS = |x| x == 'X' => { RPS::Rock });
pred!(second_paper: char => RPS = |x| x == 'Y' => { RPS::Paper });
pred!(second_scissors: char => RPS = |x| x == 'Z' => { RPS::Scissors });

group!(rps_list: char => Vec<(RPS, RPS)> = |input| {
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

pred!(lose: char => Status = |x| x == 'X' => { Status::Lose });
pred!(draw: char => Status = |x| x == 'Y' => { Status::Draw });
pred!(win: char => Status = |x| x == 'Z' => { Status::Win });

group!(correct_rps_list: char => Vec<(RPS, Status)> = |input| {
    alt!(second: char => Status = lose| draw | win);

    seq!(row: char => (RPS, Status) = f <= first, ws, s <= second, {
        (f, s)
    });

    seq!(row_end_line: char => (RPS, Status) = r <= row, end_line, { r });

    seq!(rows: char => Vec<(RPS, Status)> = rs <= * row_end_line, mr <= ? row, {
        let mut rs = rs;
        match mr {
            Some(r) => { rs.push(r); rs },
            None => rs,
        }
    });

    rows(input)
});
