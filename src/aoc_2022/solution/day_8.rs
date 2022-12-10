
use std::collections::HashMap;

use super::super::inputs::input::*;

fn to_hash(input : &str, row_len : usize) -> HashMap<(usize, usize), usize> {
    input.replace(|x| x == '\n' || x == '\r', "")
        .char_indices()
        .map(|(index, value)| {
            let col = index % row_len;
            let row = index / row_len;
            ((row, col), value as usize - 0x30)
        })
        .collect::<HashMap<_, _>>()
}

fn to_col(hash : &HashMap<(usize, usize), usize>, target : (usize, usize), row_len : usize) -> Vec<usize> {
    let (r, c) = target;

    let mut ret = vec![];
    for x in 0..row_len {
        if x != r {
            ret.push(*hash.get(&(x, c)).unwrap());
        }
    } 
    ret
}

fn to_row(hash : &HashMap<(usize, usize), usize>, target : (usize, usize), col_len : usize) -> Vec<usize> {
    let (r, c) = target;

    let mut ret = vec![];
    for x in 0..col_len {
        if x != c {
            ret.push(*hash.get(&(r, x)).unwrap());
        }
    } 
    ret
}

#[allow(dead_code)]
pub fn solve_1() {
    let row_len = 99;
    let col_len = 99;

    let input = DAY_8_1;

    let hash = to_hash(input, row_len);

    let mut seen = vec![];

    for (r, c) in (0..row_len).flat_map(|x| (0..col_len).map(move |y| (x, y))) {
        if r == 0 || c == 0 || r == row_len - 1 || c == col_len - 1 {
            seen.push((r, c));
            continue;
        }

        let target = *hash.get(&(r,c)).unwrap();

        let row = to_row(&hash, (r, c), col_len);

        let (a, b) = row.split_at(c);

        let max = a.iter().max().unwrap();

        if *max < target {
            seen.push((r,c));
            continue;
        }

        let max = b.iter().max().unwrap();

        if *max < target {
            seen.push((r,c));
            continue;
        }

        let col = to_col(&hash, (r, c), row_len);

        let (a, b) = col.split_at(r);

        let max = a.iter().max().unwrap();

        if *max < target {
            seen.push((r,c));
            continue;
        }

        let max = b.iter().max().unwrap();

        if *max < target {
            seen.push((r,c));
            continue;
        }
    }

    println!("2022 day 8:1 = {:?}", seen.len());
}

#[allow(dead_code)]
pub fn solve_2() {
    fn dist<'a>(target : usize, list : impl Iterator<Item = &'a usize> ) -> usize {
        let mut d = 1;
        for x in list {
            if *x >= target {
                return d;
            }
            d += 1;
        }
        d - 1
    }
    let row_len = 99;
    let col_len = 99;

    let input = DAY_8_1;

    let hash = to_hash(input, row_len);

    let mut max = 0;

    for (r, c) in (0..row_len).flat_map(|x| (0..col_len).map(move |y| (x, y))) {
        if r == 0 || c == 0 || r == row_len - 1 || c == col_len - 1 {
            continue;
        }

        let target = *hash.get(&(r,c)).unwrap();

        let row = to_row(&hash, (r, c), col_len);
        let col = to_col(&hash, (r, c), row_len);
        let (row_before, row_after) = row.split_at(c);
        let (col_before, col_after) = col.split_at(r);

        let x = vec! [ dist(target, row_after.iter())
                     , dist(target, row_before.iter().rev())
                     , dist(target, col_after.iter())
                     , dist(target, col_before.iter().rev()) 
                    ].into_iter().product();

        if x > max {
            max = x;
        }
    }

    println!("2022 day 8:2 = {}", max);
}