
use std::cmp::Ordering;
use motif::*;
use crate::common::parsing::*;
use super::super::inputs::input::*;

#[derive(Debug, Clone)]
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

fn cmp(pair : (&Item, &Item)) -> Ordering {
    match pair {
        (Item::Int(a), Item::Int(b)) => a.cmp(b),
        (a @ Item::Int(_), b) => cmp((&Item::List(vec![a.clone()]), b)),
        (a, b @ Item::Int(_)) => cmp((a, &Item::List(vec![b.clone()]))),
        (Item::List(a), Item::List(b)) => {
            for (a, b) in a.iter().zip(b.iter()) {
                let r = cmp((a, b));
                if r != Ordering::Equal {
                    return r;
                }
            }
            if a.len() < b.len() {
                Ordering::Less
            }
            else if a.len() == b.len() {
                Ordering::Equal
            }
            else {
                Ordering::Greater
            }
        },
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_13_1;

    let s = parse(input).into_iter()
                        .map(|(a, b)| cmp((&a, &b)))
                        .enumerate()
                        .filter(|(_, x)| *x == Ordering::Less)
                        .map(|(x, _)| x + 1)
                        .sum::<usize>();

    println!("2022 day 13:1 = {}", s);

}

#[allow(dead_code)]
pub fn solve_2() {
    let input = DAY_13_1;
    let input = format!("[[2]]\n[[6]]\n\n{}", input);

    let mut packets = parse(input.as_str()).into_iter().flat_map(|(a,b)| vec![a, b]).collect::<Vec<_>>();
    packets.sort_by(|a, b| cmp((a, b)));

    let s = packets.into_iter()
                   .enumerate()
                   .filter(|(_, x)| {
                        match x {
                            Item::List(x) if x.len() == 1 => {
                                match &x[0] {
                                    Item::List(x) if x.len() == 1 => {
                                        match &x[0] {
                                            Item::Int(2 | 6) => true,
                                            _ => false,
                                        }
                                    },
                                    _ => false,
                                }

                            },
                            _ => false,
                        }
                   })
                   .map(|(x, _)| x + 1)
                   .product::<usize>();

    println!("2022 day 13:2 = {}", s);
}