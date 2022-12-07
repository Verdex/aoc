
use bending::*;
use denest::*;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<'a, T> Linearizable<'a> for List<T> {
    fn l_next(&'a self) -> Vec<&'a Self> {
        match self {
            List::Cons(_, rest) => vec![rest],
            List::Nil => vec![],
        }
    }
}

pub fn solve_1() {

}

pub fn solve_2() {

}