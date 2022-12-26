
use std::fmt::Debug;
use std::collections::HashMap;
use super::super::inputs::input::*;

type Loc = (usize, usize);
type Map<T> = HashMap<Loc, T>;

fn display<V : Debug>(maze : &Map<V>) {
    let row_max = maze.iter().map(|((r, _), _)| *r).max().unwrap();
    let col_max = maze.iter().map(|((_, c), _)| *c).max().unwrap();

    let x = (0..=row_max).map(|r| (0..=col_max).map(|c| format!("{:^5}", format!("{:?}", *maze.get(&(r, c)).unwrap())))
                                               .collect::<Vec<_>>()
                                               .join("")
                             ).collect::<Vec<_>>()
                              .join("\n");
    println!("{:}", x);
}

fn parse(input : &str) -> Map<char> {
    let mut ret = HashMap::new();
    let lines = input.split(|x| x == '\n' || x == '\r').filter(|l| l.len() != 0);

    for (row, line) in lines.enumerate() {
        for (col, letter) in line.char_indices() {
            ret.insert((row, col), letter);
        }
    }

    ret
}

fn neighbors<T : Copy>(maze : &Map<T>, loc : Loc) -> Vec<(Loc, T)> {
    let (r, c) = loc;
    let mut ret = vec![];
    match maze.get(&(r + 1, c)) {
        Some(x) => ret.push(((r + 1, c), *x)),
        _ => { },
    }
    match maze.get(&(r, c + 1)) {
        Some(x) => ret.push(((r, c + 1), *x)),
        _ => { },
    }
    if r != 0 {
        match maze.get(&(r - 1, c)) {
            Some(x) => ret.push(((r - 1, c), *x)),
            _ => { },
        }
    }
    if c != 0 {
        match maze.get(&(r, c - 1)) {
            Some(x) => ret.push(((r, c - 1), *x)),
            _ => { },
        }
    }
    ret
}

fn valid_neighbor(current : char, neighbor : char) -> bool {
    if current == 'S' {
        neighbor == 'a'
    }
    else if neighbor == 'E' {
        current == 'z'
    }
    else {
        neighbor as i32 - 1 <= current as i32
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_12_1;
    /*let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";*/

    let mut maze = parse(input);
    let mut dist = maze.iter().map(|(loc, _)| (loc, u64::MAX)).collect::<HashMap<_, _>>();

    let src = *maze.iter().find(|(_, l)| **l == 'S').unwrap().0;
    let dest = *maze.iter().find(|(_, l)| **l == 'E').unwrap().0;

    *dist.get_mut(&src).unwrap() = 0;

    let mut q = maze.iter().collect::<Vec<_>>();

    while q.len() != 0 {
        let (&&current_loc, _) = dist.iter().filter(|(k, v)| q.iter().find(|(x, _)| x == *k).is_some()).min_by_key(|(k, v)| *v).unwrap();
        let current_letter = *maze.get(&current_loc).unwrap();

        if current_loc == dest {
            break;
        }

        let (index, _) = q.iter().enumerate().find(|(index, (x, _))| **x == current_loc).unwrap();
        q.remove(index);

        for (neighbor_loc, _) in neighbors(&maze, current_loc).iter().filter(|(_, x)| valid_neighbor(current_letter, *x)) {
            let alt = *dist.get(&current_loc).unwrap() + 1;
            if alt < *dist.get(neighbor_loc).unwrap() {
                *dist.get_mut(neighbor_loc).unwrap() = alt;
            }
        }

        println!("{}", q.len());
    }

    println!("2022 day 12:1 = {}", dist.get(&dest).unwrap());
}

#[allow(dead_code)]
pub fn solve_2() {
    //let input = DAY_12_1;
    /*let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";*/

}