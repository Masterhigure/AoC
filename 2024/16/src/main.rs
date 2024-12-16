#![allow(unused)]
use std::fs;

type Input = Vec<i32>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input) -> i32 {
    0
}

fn task_two(mut input: Input) -> i32 {
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
        assert_eq!(0, task_one(input.clone()));
        assert_eq!(0, task_two(input));
    }
}
