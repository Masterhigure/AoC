#![allow(unused)]
use std::fs;

type Input = Vec<Vec<i32>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.split(' ').map(str::parse).map(Result::unwrap).collect())
        .collect()
}

fn prev_num(v: Vec<i32>) -> i32 {
    let diff = v.iter()
        .zip(v.iter().skip(1))
        .map(|(m, n)| n-m)
        .collect::<Vec<_>>();
    if diff.iter().all(|&n| n == 0) {
        v[0]
    }
    else {
        v[0] - prev_num(diff)
    }
}

fn next_num(v: Vec<i32>) -> i32 {
    let diff = v.iter()
        .zip(v.iter().skip(1))
        .map(|(m, n)| n-m)
        .collect::<Vec<_>>();
    if diff.iter().all(|&n| n == 0) {
        v[0]
    }
    else {
        v[v.len() - 1] + next_num(diff)
    }
}

fn task_one(mut input: Input) -> i32 {
    let mut s = 0;
    for v in input {
        s += next_num(v);
    }
    s
}

fn task_two(mut input: Input) -> i32 {
    let mut s = 0;
    for v in input {
        s +=prev_num(v);
    }
    s
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
        assert_eq!(114, task_one(input.clone()));
        assert_eq!(2, task_two(input));
    }
}
