
use std::collections::HashMap;

use super::super::inputs::input::*;


fn parse(input : &str) -> HashMap<(i32, i32), char> {
    let mut ret = HashMap::new();
    let lines = input.split(|x| x == '\n' || x == '\r');

    for (row, line) in lines.enumerate() {
        for (col, letter) in line.char_indices() {
            ret.insert((row as i32, col as i32), letter);
        }
    }

    ret
}

macro_rules! jabber {
    ($row:expr, $col:expr, $letter:ident, $nexts:ident, $h:ident, $path:ident) => {
        if let Some(v) = $h.get(&($row, $col)) {
            if !$path.iter().any(|p| *p == ($row, $col)) {
                if *v == 'E' {
                    if $letter == 'z' {
                        $nexts.push(($row, $col));
                    }
                } 
                else if (*v as i32) - 1 <= $letter as i32 {
                    $nexts.push(($row, $col));
                }
            }
        }
    };
}

fn blarg(h : HashMap<(i32, i32), char>) -> impl Iterator<Item = Vec<(i32, i32)>> {
    let mut items = vec![vec![(0, 0)]];
    let mut path = vec![];
    let mut current = vec![];

    std::iter::from_fn(move || {
        while items.len() != 0 {
            current = items.pop().unwrap();
            path.push(current);
            let letter = *h.get(&current).unwrap();
            let (row, col) = current;
            let mut nexts = vec![];
            jabber!(row - 1, col, letter, nexts, h, path);
            jabber!(row, col - 1, letter, nexts, h, path);
            jabber!(row + 1, col, letter, nexts, h, path);
            jabber!(row, col + 1, letter, nexts, h, path);
            if nexts.len() == 0 {
                let ret = Some(path.clone());
                path.pop();
                return ret;
            }
            nexts.append(&mut items);
            std::mem::swap(&mut nexts, &mut items);
        }
        None 
    })
}

#[allow(dead_code)]
pub fn solve_1() {
    //let input = DAY_12_1;
    let input = "aabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let maze = parse(input);

    let b = blarg(maze).collect::<Vec<_>>();

    let maze = parse(input);
    for blet in b {
        println!("{:?}", blet.len());
        let y = blet.iter().map(|x| maze.get(x).unwrap()).collect::<String>();
        println!("{}", y);
    }
    //println!("2022 day 12:1 = {:?}", b);
}

#[allow(dead_code)]
pub fn solve_2() {
    println!("2022 day 12:2 = {}", 0);
}