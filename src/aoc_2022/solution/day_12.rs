
use std::collections::HashMap;

use super::super::inputs::input::*;

#[derive(Debug)]
struct Loc {
    row : i32,
    col : i32,
    letter : char,
}

fn parse(input : &str) -> HashMap<(i32, i32), char> {
    let mut ret = HashMap::new();
    let lines = input.split(|x| x == '\n' || x == '\r').filter(|l| l.len() != 0);

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

fn paths(maze : HashMap<(i32, i32), char>) -> impl Iterator<Item = Vec<Loc>> {
    let mut pending = vec![vec![(0,0)]];
    let mut path = vec![];
    let mut min = usize::MAX;
    std::iter::from_fn(move || {
        while pending.len() != 0 {
            let mut options = pending.pop().unwrap();
            if path.len() > min {
                path.pop();
                continue;
            }
            if options.len() == 0 {
                path.pop();
                continue;
            }
            let current = options.pop().unwrap();
            pending.push(options);
            path.push(current);
            let (row, col) = current;
            let letter = *maze.get(&current).unwrap();
            let mut nexts = vec![];
            jabber!(row - 1, col, letter, nexts, maze, path);
            jabber!(row, col - 1, letter, nexts, maze, path);
            jabber!(row + 1, col, letter, nexts, maze, path);
            jabber!(row, col + 1, letter, nexts, maze, path);
            if nexts.len() == 0 {
                let last = path.last().unwrap();
                if *maze.get(&last).unwrap() == 'E' {
                    let ret = path.iter().map(|(r, c)| Loc { row: *r, col: *c, letter: *maze.get(&(*r,*c)).unwrap() }).collect::<Vec<_>>();
                    if ret.len() < min {
                        min = ret.len();
                    }
                    path.pop();
                    return Some(ret);
                }
                else {
                    path.pop();
                }
            }
            else {
                pending.push(nexts);
            }
        }
        None
    })
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_12_1;
    /*let input = "aabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";*/

    let maze = parse(input);

    let s = paths(maze).map(|p| p.len() - 1).min().unwrap();

    println!("2022 day 12:1 = {}", s);
}

#[allow(dead_code)]
pub fn solve_2() {
    println!("2022 day 12:2 = {}", 0);
}