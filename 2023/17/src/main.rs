#![allow(unused)]
use std::fs;
use std::collections::{HashSet, HashMap};

type Input = Vec<Vec<u32>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node {
    x: usize,
    y: usize,
    c: u32,
    d: u32,
    n: u32,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct VisitedNode {
    x: usize,
    y: usize,
    d: u32,
    n: u32,
}

fn neighs(cur: Node, input: &Input) -> Vec<Node> {
    let mut neigh = Vec::new();
    if cur.x > 0 && (cur.d != 0 || cur.n < 3) && cur.d != 2 {
        let mut n = cur.clone();
        n.x -= 1;
        n.c += input[n.x][n.y];
        if cur.d == 0 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 0;
        }
        neigh.push(n);
    }
    if cur.x + 1 < input.len() && (cur.d != 2 || cur.n < 3) && cur.d != 0 {
        let mut n = cur.clone();
        n.x += 1;
        n.c += input[n.x][n.y];
        if cur.d == 2 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 2;
        }
        neigh.push(n);
    }
    if cur.y > 0 && (cur.d != 3 || cur.n < 3) && cur.d != 1 {
        let mut n = cur.clone();
        n.y -= 1;
        n.c += input[n.x][n.y];
        if cur.d == 3 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 3;
        }
        neigh.push(n);
    }
    if cur.y + 1 < input.len() && (cur.d != 1 || cur.n < 3) && cur.d != 3 {
        let mut n = cur.clone();
        n.y += 1;
        n.c += input[n.x][n.y];
        if cur.d == 1 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 1;
        }
        neigh.push(n);
    }
    neigh
}

fn neighs_two(cur: Node, input: &Input) -> Vec<Node> {
    let mut neigh = Vec::new();
    if cur.x > 0 && (cur.d != 0 || cur.n < 10) && cur.d != 2 
        && !((cur.d == 1 || cur.d == 3) && cur.n < 4 ){
        let mut n = cur.clone();
        n.x -= 1;
        n.c += input[n.x][n.y];
        if cur.d == 0 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 0;
        }
        neigh.push(n);
    }
    if cur.x + 1 < input.len() && (cur.d != 2 || cur.n < 10) && cur.d != 0 
        && !((cur.d == 1 || cur.d == 3) && cur.n < 4 ){
        let mut n = cur.clone();
        n.x += 1;
        n.c += input[n.x][n.y];
        if cur.d == 2 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 2;
        }
        neigh.push(n);
    }
    if cur.y > 0 && (cur.d != 3 || cur.n < 10) && cur.d != 1 
        && !((cur.d == 0 || cur.d == 2) && cur.n < 4 ){
        let mut n = cur.clone();
        n.y -= 1;
        n.c += input[n.x][n.y];
        if cur.d == 3 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 3;
        }
        neigh.push(n);
    }
    if cur.y + 1 < input.len() && (cur.d != 1 || cur.n < 10) && cur.d != 3 
        && !((cur.d == 0 || cur.d == 2) && cur.n < 4 ){
        let mut n = cur.clone();
        n.y += 1;
        n.c += input[n.x][n.y];
        if cur.d == 1 {
            n.n += 1;
        } else {
            n.n = 1;
            n.d = 1;
        }
        neigh.push(n);
    }
    neigh
}
fn task_one(mut input: Input) -> u32 {
    let mut nodes = vec![
        Node{ x: 0, y: 0, c: 0, d: 1, n: 0 },
        Node{ x: 0, y: 0, c: 0, d: 2, n: 0 },
    ];
    let mut visited = HashSet::new();
    loop {
        let cur = nodes.pop().unwrap();
        if !visited.insert(VisitedNode{ x: cur.x, y: cur.y, d: cur.d, n: cur.n }) {
            continue;
        }
        if cur.x + 1 == input.len() && cur.y + 1 == input[0].len() {
            return cur.c;
        }
        for mut n in neighs(cur, &input) {
            nodes.push(n);
        }
        nodes.sort_unstable_by_key(|n| 1_000_000 - n.c);
        //dbg!(cur, &nodes, "-------------------");
    }
    0
}

fn task_two(mut input: Input) -> u32 {
    let mut nodes = vec![
        Node{ x: 0, y: 0, c: 0, d: 1, n: 0 },
        Node{ x: 0, y: 0, c: 0, d: 2, n: 0 },
    ];
    let mut visited = HashSet::new();
    loop {
        let cur = nodes.pop().unwrap();
        if !visited.insert(VisitedNode{ x: cur.x, y: cur.y, d: cur.d, n: cur.n }) {
            continue;
        }
        if cur.x + 1 == input.len() && cur.y + 1 == input[0].len() {
            return cur.c;
        }
        for mut n in neighs_two(cur, &input) {
            nodes.push(n);
        }
        nodes.sort_unstable_by_key(|n| 1_000_000 - n.c);
        //dbg!(cur, &nodes, "-------------------");
    }
    0
}

fn main() {
    let input = read_input("input.txt");
    println!("Task one: {}", task_one(input.clone()));
    println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(102, task_one(input.clone()));
        assert_eq!(94, task_two(input));
    }
}
