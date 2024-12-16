#![allow(unused)]
use std::fs;

type Input = Vec<String>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect()
}

fn task_one(mut input: Input) -> i32 {
    let mut s = 0;
    for l in input {
        let mut cmds = l.split("mul(");
        for cmd in cmds {
            let Some((s1, s2)) = cmd.split_once(',') else { continue; };
            let Ok(n1) = s1.parse::<i32>() else {continue;};
            let Some((s3, _)) = s2.split_once(')') else {continue;};
            let Ok(n2) = s3.parse::<i32>() else {continue;};
            s += n1*n2;
        }
    }
    s
}

fn task_two(mut input: Input) -> i32 {
    let mut s = 0;
    let mut flag = true;
    for l in input {
        let mut cmds = l.split("mul(");
        for cmd in cmds {
            let (s1, s2) = cmd.split_once(',').unwrap_or(("0", cmd));
            let n1 = s1.parse::<i32>().unwrap_or(0);
            let (s3, s4) = s2.split_once(')').unwrap_or(("0", s2));
            let n2 = s3.parse::<i32>().unwrap_or(0);
            if flag {
                s += n1*n2;
            }
            let on = cmd.rfind("do()");
            let off = cmd.rfind("don't()");
            if flag && off.is_some() && (on.is_none() || (on.unwrap() < off.unwrap())) {
                flag = false;
            }
            if !flag && on.is_some() && (off.is_none() || (off.unwrap() < on.unwrap())) {
                flag = true;
            }
        }
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
        assert_eq!(161, task_one(vec![input[0].clone()]));
        assert_eq!(48, task_two(vec![input[1].clone()]));
    }
}
