
use bending::*;
use denest::*;

use super::super::inputs::input::*;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> From<Vec<T>> for List<T> {
    fn from(mut input : Vec<T>) -> Self {
        input.reverse();

        let mut ret = List::Nil;
        for i in input {
            ret = List::Cons(i, Box::new(ret));
        }

        ret
    }
}

impl<'a, T> Linearizable<'a> for List<T> {
    fn l_next(&'a self) -> Vec<&'a Self> {
        match self {
            List::Cons(_, rest) => vec![rest],
            List::Nil => vec![],
        }
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    use List::*;

    fn all_diff(a : char, b : char, c : char, d : char) -> bool {
        let mut x = vec![a,b,c,d];
        x.sort();
        x.dedup();
        x.len() == 4
    }

    let target : fn(&List<(usize, char)>) -> Vec<usize>
        = object_pattern!(Cons((_, a), !)
                        ; Cons((_, b), !)
                        ; Cons((_, c), !)
                        ; Cons((target, d), _) ? { all_diff(*a, *b, *c, *d) } => { *target });

    let input = DAY_6_1;
    let list : List<(usize, char)> = input.char_indices().collect::<Vec<_>>().into();
    let result = list.to_lax().flat_map(|x| target(&x)).min().unwrap() + 1;

    println!("2022 day 6:1 = {}", result);
}

#[allow(dead_code)]
pub fn solve_2() {
    use List::*;

    fn all_diff(mut x : Vec<&char>) -> bool {
        // I wonder if this can be sped up with the day 3 bit trick
        let target = x.len();
        x.sort();
        x.dedup();
        x.len() == target 
    }

    let target : fn(&List<(usize, char)>) -> Vec<usize>
        = object_pattern!(Cons((_, _1), !)
                        ; Cons((_, _2), !)
                        ; Cons((_, _3), !)
                        ; Cons((_, _4), !)
                        ; Cons((_, _5), !)
                        ; Cons((_, _6), !)
                        ; Cons((_, _7), !)
                        ; Cons((_, _8), !)
                        ; Cons((_, _9), !)
                        ; Cons((_, _10), !)
                        ; Cons((_, _11), !)
                        ; Cons((_, _12), !)
                        ; Cons((_, _13), !)
                        ; Cons((target, _14), _) 
                        ? { all_diff(vec![_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14]) } => { *target });

    let input = DAY_6_1;
    let list : List<(usize, char)> = input.char_indices().collect::<Vec<_>>().into();
    let result = list.to_lax().flat_map(|x| target(&x)).min().unwrap() + 1;

    println!("2022 day 6:2 = {}", result);
}
