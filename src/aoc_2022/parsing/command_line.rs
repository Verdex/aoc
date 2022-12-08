
use motif::*;

use super::super::data::*;

pub fn parse_command_line(input : &str) -> CommandLine {

    panic!("blarg");
}

group!(symbol: char => String = |input| {
    pred!(letter: char = |x| x.is_alphabetic());
    pred!(dot: char = |x| x == '.');
    alt!(sym_char: char = dot | letter);

    seq!(main: char => String = xs <= * sym_char, { xs.into_iter().collect::<String>() });

    main(input)
});

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

group!(cd: char => CommandLine = |input| {
    pred!(slash: char = |x| x == '/');
    seq!(cd: char = 'c', 'd', { '\0' });
    seq!(dot_dot: char = '.', '.', { '\0' });
    seq!(cd_slash: char => CommandLine = cd, ! ws, slash, { CommandLine::CdHome });
    seq!(cd_up: char => CommandLine = cd, ! ws, dot_dot, { CommandLine::CdUp });
    seq!(cd_to: char => CommandLine = cd, ! ws, s <= symbol, { CommandLine::CdTo(s)} );

    alt!(main: char => CommandLine = cd_up | cd_slash | cd_to );

    main(input)
});

group!(dir_item: char => DirItem = |input| {
    seq!(dir: char = 'd', 'i', 'r', { '\0' });
    seq!(directory: char => DirItem = dir, ! ws, s <= symbol, { DirItem::Dir(s) });
    seq!(file: char => DirItem = n <= number, ! ws, s <= symbol, { DirItem::File(n, s) });
    alt!(main: char => DirItem = directory | file);
    main(input)
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cd_should_parse_home() {
        let input = "cd /";
        let mut i = input.char_indices();
        let output = cd(&mut i).unwrap();
        assert!( matches!( output, CommandLine::CdHome ) );
    }

    #[test]
    fn cd_should_parse_up_dir() {
        let input = "cd ..";
        let mut i = input.char_indices();
        let output = cd(&mut i).unwrap();
        assert!( matches!( output, CommandLine::CdUp ) );
    }
}