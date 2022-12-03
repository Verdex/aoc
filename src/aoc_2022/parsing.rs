

use motif::*;

use super::data::*;

pub fn parse_calorie_list_list(input : &str) -> Vec<Vec<Calorie>> {

    let mut i = input.char_indices();

    let ret = number_list_list(&mut i).expect("failed to correctly parse number list list");
    
    if i.count() != 0 { panic!("failed to consume all input"); }

    ret.into_iter()
       .map(|list| list.into_iter().map(|number| Calorie(number)).collect::<Vec<_>>())
       .collect::<Vec<_>>()
}

group!(end_line: char => () = |input| {
    pred!(cr: char => () = |x| x == '\r' => { () });
    pred!(lf: char => () = |x| x == '\n' => { () });
    seq!(cr_lf: char => () = cr, lf, { () });

    alt!(main: char => () = cr_lf | lf | cr);

    main(input)
});

group!(number_list_list: char => Vec<Vec<u64>> = |input| {
    seq!(list_end_line: char => Vec<u64> = list <= number_list, end_line, { list });
    seq!(main: char => Vec<Vec<u64>> = lists <= * list_end_line, maybe_list <= ? number_list, {
        let mut lists = lists;
        match maybe_list {
            Some(list) => { lists.push(list); lists },
            None => lists,
        }
    });

    main(input)
});

group!(number_list: char => Vec<u64> = |input| {
    seq!(number_end_line: char => u64 = n <= number, end_line, { n });
    seq!(main: char => Vec<u64> = ns <= * number_end_line, n <= ? number, {
        let mut ns = ns;
        match n {
            Some(n) => { ns.push(n); ns },
            None => ns,
        }
    });

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

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn number_should_parse_number() -> Result<(), MatchError> {
        let input = "100";
        let mut i = input.char_indices();
        let output = number(&mut i)?;

        assert_eq!( output, 100);
        assert_eq!( i.count(), 0 );

        Ok(())
    }

    #[test]
    fn number_list_list_should_parse_lists() -> Result<(), MatchError> {
        let input = "100
200
300

400
500

600";

        let mut i = input.char_indices();

        let output = number_list_list(&mut i)?;

        assert_eq!( output.len(), 3 );
        assert_eq!( output[0], [100, 200, 300] );
        assert_eq!( output[1], [400, 500] );
        assert_eq!( output[2], [600] );
        assert_eq!(i.count(), 0);

        Ok(())
    }
}