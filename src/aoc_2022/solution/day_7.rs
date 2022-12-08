
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
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let command_line = parse_command_line(input);

    println!("2022 day 7:1 = {:?}", output);
}

#[allow(dead_code)]
pub fn solve_2() {

}