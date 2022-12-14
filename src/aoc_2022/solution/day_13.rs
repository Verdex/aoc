
use motif::*;
use crate::common::parsing::*;

enum Item {
    Int(i32),
    List(Vec<Item>),
}

pred!(l: char = |x| x == '[');
pred!(r: char = |x| x == ']');
pred!(c: char = |x| x == ',');

group!(packet: char => Item = |input| {
    seq!(number_comma: char => i64 = n <= number, c, { n });
    Err(MatchError::Fatal(0))
});

#[allow(dead_code)]
pub fn solve_1() {

}

#[allow(dead_code)]
pub fn solve_2() {

}