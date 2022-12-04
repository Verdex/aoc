
use motif::*;

use super::super::data::*;

pub fn parse_section_pair_list( input : &str ) -> Vec<(Section, Section)> {
    let mut i = input.char_indices();
    let output = section_pair_list(&mut i).unwrap();
    if i.count() != 0 { panic!("all input not consumed"); }
    output
}

pred!(dash: char => () = |x| x == '-' => { () });
pred!(comma: char => () = |x| x == ',' => { () });

group!(end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(number: char => u64 = |input| {
    pred!(digit: char = |x| x.is_numeric());
    seq!(digits: char => u64 = d <= digit, ds <= * digit, { 
        let mut ds = ds;
        ds.insert(0, d);
        ds.iter().collect::<String>().parse::<u64>().expect("already parsed as valid number")
    });

    digits(input)
});

group!(section_pair_list: char => Vec<(Section, Section)> = |input| {
    seq!(section: char => Section = start <= number, ! dash, end <= ! number, {
        Section { start, end }
    });
    seq!(pair: char => (Section, Section) = a <= section, ! comma, b <= ! section, {
        (a, b)
    });
    seq!(pair_end: char => (Section, Section) = p <= pair, end_line, { p });
    seq!(main: char => Vec<(Section, Section)> = ps <= * pair_end, mp <= ? pair, {
        let mut ps = ps;
        match mp {
            Some(p) => { ps.push(p); ps },
            None => ps,
        }
    });

    main(input)
});