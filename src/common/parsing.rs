
use motif::*;

pred!(pub ws: char = |x| x == ' ');

group!(pub end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(pub number: char => i64 = |input| {
    pred!(digit: char = |x| x.is_numeric());
    seq!(digits: char => i64 = d <= digit, ds <= * digit, { 
        let mut ds = ds;
        ds.insert(0, d);
        ds.iter().collect::<String>().parse::<i64>().expect("already parsed as valid number")
    });

    digits(input)
});
