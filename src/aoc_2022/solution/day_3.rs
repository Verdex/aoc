

use super::super::inputs::input::*;

fn to_char(x : u32) -> char {
    match x {
        1..=26 => char::from_u32(x + 96).unwrap(),
        27..=52 => char::from_u32(x + 64 - 26).unwrap(),
        _ => panic!("out of range"),
    }
}
fn lower(x : char) -> u32 { x as u32 - 96 }
fn upper(x : char) -> u32 { x as u32 - 64 }
fn priority(x : char) -> u32 {
    if x.is_lowercase() {
        lower(x)
    }
    else {
        upper(x) + 26
    }
}
fn gcd(mut a : u128, mut b : u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn is_prime(x : u128) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn find_target(primes : &[u128], input : (&str, &str)) -> usize {
    let (a, b) = input;
    let mut a = a.chars().collect::<Vec<_>>();
    a.sort();
    a.dedup();
    let mut b = b.chars().collect::<Vec<_>>();
    b.sort();
    b.dedup();
    let a : u128 = a.into_iter().map(priority).map(|x| primes[(x-1) as usize]).product();
    let b : u128 = b.into_iter().map(priority).map(|x| primes[(x-1) as usize]).product();
    let target = gcd(a, b);
    let (x, _) = primes.iter().enumerate().find(|(_, x)| **x == target).unwrap();
    x+1
    //to_char((x + 1).try_into().unwrap())
}


pub fn solve_1() {

    let primes = (2..).filter(|x| is_prime(*x)).take(52).collect::<Vec<u128>>();

    let result : usize = DAY_3_1.split("\r\n").map(|x| x.split_at(x.len() / 2))
        .map(|x| find_target(&primes, x))
        .sum();

    /*let input = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let result : usize = input.split("\n").map(|x| x.split_at(x.len() / 2))
        .map(|x| find_target(&primes, x))
        .sum();*/
                     
    println!("2022 day 3:1 = {}", result);
}