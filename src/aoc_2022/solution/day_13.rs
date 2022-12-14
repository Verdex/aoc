
use motif::*;
use crate::common::parsing::*;

#[derive(Debug)]
enum Item {
    Int(i64),
    List(Vec<Item>),
}

pred!(l: char = |x| x == '[');
pred!(r: char = |x| x == ']');
pred!(c: char = |x| x == ',');

group!(packet: char => Item = |input| {
    seq!(number_item: char => Item = n <= number, { Item::Int(n) });
    alt!(item: char => Item = number_item | list);
    seq!(item_comma: char => Item = i <= item, c, { i });
    seq!(items: char => Vec<Item> = xs <= * item_comma, mx <= ? item, {
        let mut xs = xs;
        match mx {
            Some(x) => { xs.push(x); xs },
            None => xs,
        }
    });
    seq!(list: char => Item = l, xs <= items, r, { Item::List(xs) });
    list(input)
});


group!(packet_pairs: char => Vec<(Item, Item)> = |input| {
    seq!(packet_pair: char => (Item, Item) = a <= packet, end_line, b <= packet, ? end_line, { (a, b) });
    seq!(packet_pair_end: char => (Item, Item) = p <= packet_pair, end_line, { p });
    seq!(pairs: char => Vec<(Item, Item)> = ps <= * packet_pair_end, mp <= ? packet_pair, { 
        let mut ps = ps;
        match mp {
            Some(p) => { ps.push(p); ps },
            None => ps,
        }
    });

    pairs(input)
});

fn parse(input : &str) -> Vec<(Item, Item)> {
    let mut i = input.char_indices();
    let output = packet_pairs(&mut i).unwrap();
    if i.count() != 0 { panic!("not all input consumed"); }
    output
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    let pairs = parse(input);

    println!("{:?}", pairs.last().unwrap());

}

#[allow(dead_code)]
pub fn solve_2() {

}