
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Calorie(pub u64);

#[derive(Debug, Clone, Copy)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Status {
    Win,
    Draw,
    Lose
}

#[derive(Debug)]
pub struct Section {
    pub start : u64,
    pub end : u64,
}

#[derive(Debug)]
pub struct CraneInstruction {
    pub count : usize,
    pub src : usize,
    pub dest : usize,
}

#[derive(Debug)]
pub struct CraneScenario {
    pub stacks : HashMap<usize, Vec<char>>,
    pub instrs : Vec<CraneInstruction>, 
}

#[derive(Debug)]
pub enum DirItem {
    File(usize, String),
    Dir(String),
}

#[derive(Debug)]
pub enum CommandLine {
    CdUp,
    CdTo(String),
    CdHome,
    Ls()
}