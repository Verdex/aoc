
use bending::*;
use denest::*;

use super::super::data::*;
use super::super::inputs::input::*;
use super::super::parsing::command_line::*;

#[derive(Debug)]
enum Tree {
    Leaf(usize, String),
    Node(Vec<Tree>, String),
}

impl<'a> Linearizable<'a> for Tree {
    fn l_next(&'a self) -> Vec<&'a Self> {
        match self {
            Tree::Leaf(_, _) => vec![],
            Tree::Node(children, _) => children.iter().map(|x| x).collect::<Vec<_>>(),
        }
    }
}

fn leaf_values(input : &Tree) -> usize {
    let f : fn(&Tree) -> Vec<usize>
        = object_pattern!(Tree::Leaf(v, _) => { *v });
    input.to_lax().flat_map(|x| f(x)).sum()
}

fn problem_1_target_directories_sum(input : &Tree) -> usize {
    let f : fn(&Tree) -> Vec<usize>
        = object_pattern!(Tree::Node(children, _) & { let total = children.iter().map(|x| leaf_values(x)).sum(); } => { total });

    input.to_lax().flat_map(|x| f(x)).filter(|x| *x <= 100000).sum()
}

fn problem_2_target_directory_size(input : &Tree) -> usize {
    let total_space = 70000000; 
    let required_space = 30000000;

    let f : fn(&Tree) -> Vec<usize>
        = object_pattern!(Tree::Node(children, _) & { let total = children.iter().map(|x| leaf_values(x)).sum(); } => { total });

    let max = input.to_lax().flat_map(|x| f(x)).max().unwrap();

    let free_space = total_space - max;

    let target_space = required_space - free_space;

    input.to_lax().flat_map(|x| f(x)).filter(|x| *x >= target_space).min().unwrap()
}

fn to_tree(input : &mut Vec<CommandLine>, current_dir : String) -> Tree {
    let mut children = vec![];
    while input.len() != 0 {
        let item = input.pop().unwrap();
        match item {
            CommandLine::CdHome => unreachable!(),
            CommandLine::CdUp => { return Tree::Node(children, current_dir); },
            CommandLine::CdTo(dir) => children.push(to_tree(input, dir)),
            CommandLine::Ls(dir_items) => {
                for dir_item in dir_items {
                    match dir_item {
                        DirItem::File(size, name) => children.push(Tree::Leaf(size, name)),
                        _ => { }, 
                    }
                }
            },
        }
    }

    Tree::Node(children, current_dir)
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = DAY_7_1;
    let mut command_line = parse_command_line(input);
    command_line.remove(0);
    command_line.reverse();
    let tree = to_tree(&mut command_line, "/".into());

    let output = problem_1_target_directories_sum(&tree);

    println!("2022 day 7:1 = {}", output);
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = DAY_7_1;
    let mut command_line = parse_command_line(input);
    command_line.remove(0);
    command_line.reverse();
    let tree = to_tree(&mut command_line, "/".into());

    let output = problem_2_target_directory_size(&tree);

    println!("2022 day 7:2 = {}", output);
}