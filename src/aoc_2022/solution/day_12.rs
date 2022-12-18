
use std::collections::HashMap;
use std::fmt::Debug;
use super::super::inputs::input::*;

/*#[derive(Debug, Clone)]
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
    ($row:expr, $col:expr, $letter:ident, $nexts:ident, $h:ident, $path:ident, $reject:ident) => {
        if let Some(v) = $h.get(&($row, $col)) {
            if !$reject.iter().any(|p| *p == ($row, $col)) {
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
        }
    };
}

fn paths(maze : HashMap<(i32, i32), char>, src : (i32, i32), dest : (i32, i32)) -> impl Iterator<Item = Vec<Loc>> {
    fn dist_sq(a : (i32, i32), b : (i32, i32)) -> i32 {
        let (a_r, a_c) = a;
        let (b_r, b_c) = b;
        let r = b_r - a_r;
        let c = b_c - a_c;
        (r * r) + (c * c)
    }
    let mut blessed = vec![];
    let mut reject = vec![];
    let mut pending = vec![vec![src]];
    let mut path = vec![];
    let mut min = 650;
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
            jabber!(row - 1, col, letter, nexts, maze, path, reject);
            jabber!(row, col - 1, letter, nexts, maze, path, reject);
            jabber!(row + 1, col, letter, nexts, maze, path, reject);
            jabber!(row, col + 1, letter, nexts, maze, path, reject);
            nexts.sort_by(|a, b| dist_sq(*a, dest).cmp(&dist_sq(*b, dest)));
            nexts.reverse();
            if nexts.len() == 0 {
                let last = path.last().unwrap();
                if *maze.get(&last).unwrap() == 'E' {
                    blessed.append(&mut path.clone());
                    let ret = path.iter().map(|(r, c)| Loc { row: *r, col: *c, letter: *maze.get(&(*r,*c)).unwrap() }).collect::<Vec<_>>();
                    if ret.len() < min {
                        min = ret.len();
                    }
                    path.pop();
                    return Some(ret);
                }
                else {
                    match path.pop() {
                        Some(r) if !blessed.iter().any(|b| *b == r) => { reject.push(r); },
                        _ => {},
                    }
                }
            }
            else {
                pending.push(nexts);
            }
        }
        None
    })
}


/*fn paths(maze : HashMap<(i32, i32), char>, src : (i32, i32), dest : (i32, i32)) -> impl Iterator<Item = Vec<Loc>> {
    fn dist_sq(a : (i32, i32), b : (i32, i32)) -> i32 {
        let (a_r, a_c) = a;
        let (b_r, b_c) = b;
        let r = b_r - a_r;
        let c = b_c - a_c;
        (r * r) + (c * c)
    }
    let mut reject = vec![];
    let mut pending = vec![vec![src]];
    let mut path = vec![];
    let mut min = usize::MAX;
    std::iter::from_fn(move || {
        while pending.len() != 0 {
            let mut options = pending.pop().unwrap();
            if path.len() > min {
                match path.pop() {
                    Some(r) => { reject.push(r); },
                    _ => {},
                }
                continue;
            }
            if options.len() == 0 {
                match path.pop() {
                    Some(r) => { reject.push(r); },
                    _ => {},
                }
                continue;
            }
            let current = options.pop().unwrap();
            pending.push(options);
            path.push(current);
            let (row, col) = current;
            let letter = *maze.get(&current).unwrap();
            let mut nexts = vec![];
            jabber!(row - 1, col, letter, nexts, maze, path, reject);
            jabber!(row, col - 1, letter, nexts, maze, path, reject);
            jabber!(row + 1, col, letter, nexts, maze, path, reject);
            jabber!(row, col + 1, letter, nexts, maze, path, reject);
            nexts.sort_by(|a, b| dist_sq(*a, dest).cmp(&dist_sq(*b, dest)));
            if nexts.len() > 1 {
                assert!( dist_sq(nexts[0], dest) <= dist_sq(nexts[1], dest) );
            }
            nexts.reverse();
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
                    match path.pop() {
                        Some(r) => { reject.push(r); },
                        _ => {},
                    }
                }
            }
            else {
                pending.push(nexts);
            }
        }
        None
    })
}*/

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_12_1;
    /*let input = "aabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";*/

    let mut maze = parse(input);

    let src = *maze.iter().find(|(_, l)| **l == 'S').unwrap().0;
    let dest = *maze.iter().find(|(_, l)| **l == 'E').unwrap().0;

    *maze.get_mut(&src).unwrap() = ('a' as u8 - 1) as char;

    let mut m = usize::MAX;
    for x in paths(maze, src, dest).map(|p| p.len() - 1) {
        if x < m {
            m = x;
        }
        println!("{}:{}", m, x);
    }

    //println!("2022 day 12:1 = {}", s);
}

#[allow(dead_code)]
pub fn solve_2() {
    println!("2022 day 12:2 = {}", 0);
}*/

fn display<V : Debug>(maze : &HashMap<(usize, usize), V>) {
    let row_max = maze.iter().map(|((r, _), _)| *r).max().unwrap();
    let col_max = maze.iter().map(|((_, c), _)| *c).max().unwrap();

    let x = (0..=row_max).map(|r| (0..=col_max).map(|c| format!("{:^5}", format!("{:?}", *maze.get(&(r, c)).unwrap())))
                                               .collect::<Vec<_>>()
                                               .join("")
                             ).collect::<Vec<_>>()
                              .join("\n");
    println!("{:}", x);
}

fn display2(maze : &HashMap<(usize, usize), Mazelet>) {
    let z = maze.iter().map(|(k, x)| (*k, match x {
        Mazelet::Unmapped(l) => char::from_u32(*l).unwrap().to_string(),
        Mazelet::Mapped { letter, dist } => dist.to_string(),
        Mazelet::Start => "S".to_string(),
        Mazelet::End => "E".to_string(),
    })).collect::<HashMap<_, _>>();

    display(&z);
}

fn parse(input : &str) -> HashMap<(usize, usize), char> {
    let mut ret = HashMap::new();
    let lines = input.split(|x| x == '\n' || x == '\r').filter(|l| l.len() != 0);

    for (row, line) in lines.enumerate() {
        for (col, letter) in line.char_indices() {
            ret.insert((row, col), letter);
        }
    }

    ret
}

fn neighbors<T>(maze : &HashMap<(usize, usize), T>, loc : (usize, usize)) -> Vec<(usize, usize)> {
    let (r, c) = loc;
    let mut ret = vec![];
    match maze.get(&(r + 1, c)) {
        Some(_) => ret.push((r + 1, c)),
        _ => { },
    }
    match maze.get(&(r, c + 1)) {
        Some(_) => ret.push((r, c + 1)),
        _ => { },
    }
    if r != 0 {
        match maze.get(&(r - 1, c)) {
            Some(_) => ret.push((r - 1, c)),
            _ => { },
        }
    }
    if c != 0 {
        match maze.get(&(r, c - 1)) {
            Some(_) => ret.push((r, c - 1)),
            _ => { },
        }
    }
    ret
}

#[derive(Debug)]
enum Mazelet {
    Unmapped(u32),
    Mapped{ letter: u32, dist : u32 },
    Start,
    End,
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

    let src = *maze.iter().find(|(_, l)| **l == 'S').unwrap().0;
    let dest = *maze.iter().find(|(_, l)| **l == 'E').unwrap().0;

    let mut maze = maze.into_iter().map(|x| match x {
                                            (key, letter @ 'a'..='z') => (key, Mazelet::Unmapped(letter as u32)),
                                            (key, 'E') => (key, Mazelet::End),
                                            (key, 'S') => (key, Mazelet::Start),
                                            _ => unreachable!(),
                                        })
                                .collect::<HashMap<_, _>>();

    let targets = neighbors(&maze, dest).into_iter()
                                        .filter(|loc| {
                                            if let Some(Mazelet::Unmapped(x)) = maze.get(loc){
                                                *x == 'z' as u32
                                            } else { 
                                                false
                                            }
                                        }).collect::<Vec<_>>();

    for t in targets {
        *maze.get_mut(&t).unwrap() = Mazelet::Mapped { letter: 'z' as u32, dist: 1 };
    }

    let mut unmapped = 0;

    loop {

        let targets = maze.iter().filter(|(_, v)| matches!( v, Mazelet::Mapped { .. }))
                                .map(|(k, _)| (*k, neighbors(&maze, *k)))
                                .collect::<Vec<_>>();
        
        for (mapped, neighbors) in targets {
            let (letter, dist) = match maze.get(&mapped).unwrap() {
                Mazelet::Mapped { letter, dist } => (*letter, *dist),
                _ => unreachable!(),
            };
            for neighbor in neighbors {
                if let Some(Mazelet::Unmapped(x)) = maze.get(&neighbor) {
                    if letter - 1 <= *x {
                        maze.insert(neighbor, Mazelet::Mapped { letter: *x, dist: dist + 1 });
                    }
                }
                if let Some(Mazelet::Mapped { letter: x, dist: old_dist }) = maze.get(&neighbor) {
                    if *old_dist > dist + 1 && letter - 1 <= *x {
                        maze.insert(neighbor, Mazelet::Mapped { letter: *x, dist: dist + 1 });
                    }
                }
            }
        }

        let temp = maze.iter().filter(|(_, v)| matches!(v, Mazelet::Unmapped(_))).count();
        if temp == unmapped {
            break;
        }
        else {
            unmapped = temp;
        }
    }

    display2(&maze);

    for ns in neighbors(&maze, src) {
        println!("{:?}", maze.get(&ns).unwrap());
    }
    //println!("2022 day 12:1 = {}", s);
}

#[allow(dead_code)]
pub fn solve_2() {
    println!("2022 day 12:2 = {}", 0);
}