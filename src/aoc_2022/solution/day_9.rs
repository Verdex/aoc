
use super::super::inputs::input::*;

type Loc = (i32, i32);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir { R, L, U, D }

fn parse_dir(input : &str) -> Dir {
    match input {
        "R" => Dir::R,
        "L" => Dir::L,
        "U" => Dir::U,
        "D" => Dir::D,
        _ => unreachable!(),
    }
}

fn follow(head : Loc, tail : Loc) -> Loc {
    let (hx, hy) = head;
    let (tx, ty) = tail;
    let dx = hx - tx;
    let dy = hy - ty;

    if [dx.abs(), dy.abs()].into_iter().max().unwrap() <= 1 {
        tail
    }
    else if dx.abs() == 2 && dy == 0 {
        (tx + dx / 2, ty)
    }
    else if dy.abs() == 2 && dx == 0 {
        (tx, ty + dy / 2)
    }
    else if dx.abs() == 2 && dy.abs() == 1 {
        (tx + dx / 2, ty + dy)
    }
    else if dy.abs() == 2 && dx.abs() == 1 {
        (tx + dx, ty + dy / 2)
    }
    else if dx.abs() == 2 && dy.abs() == 2 {
        (tx + dx / 2, ty + dy / 2) 
    }
    else {
        panic!("head = {:?}; tail = {:?}", head, tail);
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_9_1;
    let input = input.replace("\r", "");
    let instrs = input.split("\n")
                      .map(|x| x.split(" ").collect::<Vec<_>>())
                      .map(|x| (parse_dir(x[0]), x[1].parse::<usize>().unwrap()))
                      .collect::<Vec<_>>();

    let mut x = 0;
    let mut y = 0;
    let mut tail = (0, 0);
    
    let mut visits = vec![(0, 0)];

    for instr in instrs {
        use Dir::*;
        let (dir, dist) = instr;
        for _ in 0..dist {
            match dir {
                R => { x += 1; },
                L => { x -= 1; },
                U => { y += 1; },
                D => { y -= 1; },
            }
            
            tail = follow((x, y), tail);

            if visits.iter().find(|&&x| x == tail).is_none() {
                visits.push(tail);
            }
        }
    }

    println!("2022 day 9:1 = {}", visits.len());
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = DAY_9_1;
    let input = input.replace("\r", "");
    let instrs = input.split("\n")
                      .map(|x| x.split(" ").collect::<Vec<_>>())
                      .map(|x| (parse_dir(x[0]), x[1].parse::<usize>().unwrap()))
                      .collect::<Vec<_>>();

    let mut x = 0;
    let mut y = 0;
    let mut tail = [(0, 0); 9];
    
    let mut visits = vec![(0, 0)];

    for instr in instrs {
        use Dir::*;
        let (dir, dist) = instr;
        for _ in 0..dist {
            match dir {
                R => { x += 1; },
                L => { x -= 1; },
                U => { y += 1; },
                D => { y -= 1; },
            }

            for index in 0..tail.len() {
                if index == 0 {
                    tail[0] = follow((x,y), tail[0]);
                } 
                else {
                    tail[index] = follow(tail[index - 1], tail[index]);
                }
            }
            
            if visits.iter().find(|&&x| x == tail[8]).is_none() {
                visits.push(tail[8]);
            }
        }
    }

    println!("2022 day 9:2 = {}", visits.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn follow_should_handle_same() {
        let output = follow((1,1), (1,1));
        assert_eq!( output, (1, 1) );
    }

    #[test]
    fn follow_should_handle_adjacent() {
        let output = follow((1,0), (1,1));
        assert_eq!( output, (1, 1) );
        let output = follow((0,1), (1,1));
        assert_eq!( output, (1, 1) );
        let output = follow((0,0), (1,1));
        assert_eq!( output, (1, 1) );
        let output = follow((1,2), (1,1));
        assert_eq!( output, (1, 1) );
        let output = follow((2,1), (1,1));
        assert_eq!( output, (1, 1) );
        let output = follow((2,2), (1,1));
        assert_eq!( output, (1, 1) );
    }

    #[test]
    fn follow_should_handle_diagional() {
        let output = follow((4,3), (2,2));
        assert_eq!( output, (3, 3) );
        let output = follow((3,4), (2,2));
        assert_eq!( output, (3, 3) );
    }
}