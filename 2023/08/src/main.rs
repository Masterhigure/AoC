#![allow(unused)]
use std::fs;
use std::collections::HashMap;

type Input = (Vec<usize>, HashMap<String, [String; 2]>);

fn read_input(filename: &str) -> Input {
    let binding = fs::read_to_string(filename)
        .unwrap();
    let (path, nodes) = binding.split_once("\n\n").unwrap();
    let path = path.chars().map(|c| if c == 'L' {0} else {1}).collect::<Vec<_>>();
    let nodes = nodes.split('\n')
        .map(|l| {
            let (name, nexts) = l.split_once(" = (").unwrap();
            let nexts = nexts.chars();
            let left = nexts.clone().take(3).collect::<String>();
            let right = nexts.skip(5).take(3).collect::<String>();
            (name.to_string(), [left.clone(), right.clone()])
    }).collect();
    (path, nodes)
}

fn task_one(mut input: Input) -> usize {
    let path = input.0;
    let nodes = input.1;
    let mut steps = 0;
    let mut curr = "AAA".to_string();
    while curr != "ZZZ".to_string() {
        curr = nodes.get(&curr).unwrap()[path[steps % path.len()]].clone();
        steps += 1;
    }
    steps
}

fn gcd(mut m: usize, mut n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

fn lcm(m: usize, n: usize) -> usize {
    (m*n) / gcd(m, n)
}

fn task_two(mut input: Input) -> usize {
    let path = input.0;
    let nodes = input.1;
    let mut starts = nodes.keys()
        .filter(|s| s.chars().rev().next().unwrap() == 'A')
        .cloned().collect::<Vec<_>>();
    let mut collected = 1;
    for mut curr in starts {
        let mut steps = 0;
        while curr.chars().rev().next().unwrap() != 'Z' {
            curr = nodes.get(&curr).unwrap()[path[steps % path.len()]].clone();
            steps += 1;
        }
        collected = lcm(collected, steps);
    }
    collected
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
        let input1 = read_input("example_input.txt");
        let input2 = read_input("example_input_2.txt");
        assert_eq!(2, task_one(input1.clone()));
        assert_eq!(6, task_one(input2.clone()));
        let input3 = read_input("example_input_3.txt");
        assert_eq!(6, task_two(input3));
    }
}
