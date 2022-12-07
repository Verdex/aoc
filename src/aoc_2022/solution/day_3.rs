

use super::super::inputs::input::*;

fn power( input : char ) -> u64 {
    1 << priority(input)
}
fn bag_to_bits( input : &str ) -> u64 {
    input.chars().map(power).fold(0u64, |a, b| a | b)
}
fn bits_to_priority(input : u64) -> u64 {
    for x in 1..=52 {
        if (1 << x) & input != 0 {
            return x;
        }
    }
    panic!("didn't find priority of bits")
}
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

fn find_group_target(primes : &[u128], input : (&str, &str, &str)) -> usize {
    fn split_contents(primes : &[u128], input : &str ) -> (u128, u128) {
        let mut input = input.chars().collect::<Vec<_>>();
        input.sort();
        input.dedup();
        let input = input.into_iter().collect::<String>();
        let (a, b) = input.split_at(input.len() / 2);
        let a = a.chars().map(priority).map(|x| primes[(x-1) as usize]).product();
        let b = b.chars().map(priority).map(|x| primes[(x-1) as usize]).product();
        (a,b)
    }

    fn split_gcd(a : (u128, u128), b : (u128, u128)) -> u128 {
        let (a1, a2) = a;
        let (b1, b2) = b;
        let t1 = gcd(a1, b1);
        let t2 = gcd(a1, b2);
        let t3 = gcd(a2, b1);
        let t4 = gcd(a2, b2);

        t1 * t2 * t3 * t4
    }

    let (a, b, c) = input;

    let a = split_contents(primes, a);
    let b = split_contents(primes, b);
    let c = split_contents(primes, c);


    let a_and_b = split_gcd(a, b);
    let b_and_c = split_gcd(b, c);
    let target = gcd(a_and_b, b_and_c);
    let (x, _) = primes.iter().enumerate().find(|(_, x)| **x == target).unwrap();
    x+1
    //to_char((x + 1).try_into().unwrap())
}

pub fn solve_1() {

    let primes = (2..).filter(|x| is_prime(*x)).take(52).collect::<Vec<u128>>();

    let result : usize = DAY_3_1.split("\r\n").map(|x| x.split_at(x.len() / 2))
        .map(|x| find_target(&primes, x))
        .sum();

    println!("2022 day 3:1 = {}", result);
}

pub fn solve_2() {
    let primes = (2..).filter(|x| is_prime(*x)).take(52).collect::<Vec<u128>>();

    let input = DAY_3_1;
    let lines = input.split("\r\n").collect::<Vec<_>>();
    let result : usize = lines.iter().zip(lines.iter().skip(1))
        .zip(lines.iter().skip(2))
        .map(|x| (*x.0.0, *x.0.1, *x.1))
        .enumerate()
        .filter(|(i, _)| i % 3 == 0)
        .map(|(_, x)| x)
        .map(|x| find_group_target(&primes, x))
        .sum();

    println!("2022 day 3:2 = {}", result);
}

#[allow(dead_code)]
pub fn solve_1_2() {
    let input = DAY_3_1;
    let result = input.split("\r\n")
                      .map(|x| x.split_at(x.len() / 2))
                      .map(|(a,b)| bits_to_priority(bag_to_bits(a) & bag_to_bits(b)))
                      .sum::<u64>();

    println!("2022 day 3:1 v2 = {}", result);
}

#[allow(dead_code)]
pub fn solve_2_2() {

    let input = DAY_3_1;
    let lines = input.split("\r\n").collect::<Vec<_>>();
    let result = lines.iter().zip(lines.iter().skip(1))
        .zip(lines.iter().skip(2))
        .map(|x| (*x.0.0, *x.0.1, *x.1))
        .enumerate()
        .filter(|(i, _)| i % 3 == 0)
        .map(|(_, x)| x)
        .map(|(a,b,c)| bits_to_priority(bag_to_bits(a) & bag_to_bits(b) & bag_to_bits(c)))
        .sum::<u64>();

    println!("2022 day 3:2 v2 = {}", result);
}