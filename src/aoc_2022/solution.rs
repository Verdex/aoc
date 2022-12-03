
use super::inputs::input::*;
use super::parsing;
use super::data::*;

pub fn solve_day_1_2() {

    let calories = parsing::calorie::parse_calorie_list_list(DAY_1_1);
    let mut results : Vec<u64> = calories.into_iter()
        .map(|x| x.into_iter().map(|y| y.0).sum())
        .collect::<Vec<_>>();
    
    results.sort();
    results.reverse();

    let result : u64 = results.into_iter().take(3).sum();
    println!("2022 day 1:2 = {}", result);
}

pub fn solve_day_2_1() {
    use RPS::*;

    fn selection_score(choice : &RPS) -> u64 {
        match choice {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn status_score(opponent : &RPS, you : &RPS) -> u64 {
        match (opponent, you) {
            (Rock, Paper) => 6,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,

            (Paper, Rock) => 0,
            (Scissors, Paper) => 0,
            (Rock, Scissors) => 0,

            _ => 3,
        }
    }

    let guide = parsing::rps::parse_rps_list(DAY_2_1);
    let result : u64 = guide.into_iter()
                      .map(|(opponent, you)| selection_score(&you) + status_score(&opponent, &you))
                      .sum();

    println!("2022 day 2:1 = {}", result);
}

pub fn solve_day_2_2() {
    use RPS::*;
    use Status::*;

    fn selection_score(choice : &RPS) -> u64 {
        match choice {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn status_score(opponent : &RPS, you : &RPS) -> u64 {
        match (opponent, you) {
            (Rock, Paper) => 6,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,

            (Paper, Rock) => 0,
            (Scissors, Paper) => 0,
            (Rock, Scissors) => 0,

            _ => 3,
        }
    }

    fn status_to_play(opponent : &RPS, goal : &Status) -> RPS {
        match (opponent, goal) {
            (o, Draw) => *o,

            (Rock, Lose) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Lose) => Paper,

            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
        }
    }

    let guide = parsing::rps::parse_rps_list_correctly(DAY_2_1);
    let result : u64 = guide.into_iter()
                      .map(|(opponent, goal)| {
                        let you = status_to_play(&opponent, &goal);
                        selection_score(&you) + status_score(&opponent, &you)
                      })
                      .sum();

    println!("2022 day 2:2 = {}", result);
}

pub fn solve_day_3_1() {

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
            let mut temp = b;
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
        let (x, _) = primes.iter().enumerate().find(|(i, x)| **x == target).unwrap();
        x+1
        //to_char((x + 1).try_into().unwrap())
    }

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