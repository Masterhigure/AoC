#![allow(unused)]
use std::fs;

type Input = Vec<String>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect()
}

fn task_one(mut i: Input) -> u32 {
    let mut sum = 0;
    for s in i {
        let t = s.chars().filter(char::is_ascii_digit).next().unwrap().to_digit(10).unwrap();
        let o = s.chars().rev().filter(char::is_ascii_digit).next().unwrap().to_digit(10).unwrap();
        sum += 10*t + o;
    }
    sum
}

fn start_digit_pattern(s: &str) -> bool {
    s.starts_with(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
        | s.starts_with("one")
        | s.starts_with("two")
        | s.starts_with("three")
        | s.starts_with("four")
        | s.starts_with("five")
        | s.starts_with("six")
        | s.starts_with("seven")
        | s.starts_with("eight")
        | s.starts_with("nine")
}

fn end_digit_pattern(s: &str) -> bool {
    s.ends_with(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
        | s.ends_with("one")
        | s.ends_with("two")
        | s.ends_with("three")
        | s.ends_with("four")
        | s.ends_with("five")
        | s.ends_with("six")
        | s.ends_with("seven")
        | s.ends_with("eight")
        | s.ends_with("nine")
}

fn task_two(mut i: Input) -> u32 {
    let mut sum = 0;
    for s in i {
        let mut s1 = s.clone();
        while !start_digit_pattern(&s1) {
            s1.remove(0);
        }
        while !end_digit_pattern(&s1) {
            s1.pop();
        }
        if let Some(t) = s1.chars().next().unwrap().to_digit(10)
        {
            sum += 10*t;
        }
        else if s1.starts_with("one") {
            sum += 10;
        }
        else if s1.starts_with("two") {
            sum += 20;
        }
        else if s1.starts_with("three") {
            sum += 30;
        }
        else if s1.starts_with("four") {
            sum += 40;
        }
        else if s1.starts_with("five") {
            sum += 50;
        }
        else if s1.starts_with("six") {
            sum += 60;
        }
        else if s1.starts_with("seven") {
            sum += 70;
        }
        else if s1.starts_with("eight") {
            sum += 80;
        }
        else if s1.starts_with("nine") {
            sum += 90;
        }
        else {
            unreachable!();
        }
        if let Some(o) = s1.chars().rev().next().unwrap().to_digit(10)
        {
            sum += o;
        }
        else if s1.ends_with("one") {
            sum += 1;
        }
        else if s1.ends_with("two") {
            sum += 2;
        }
        else if s1.ends_with("three") {
            sum += 3;
        }
        else if s1.ends_with("four") {
            sum += 4;
        }
        else if s1.ends_with("five") {
            sum += 5;
        }
        else if s1.ends_with("six") {
            sum += 6;
        }
        else if s1.ends_with("seven") {
            sum += 7;
        }
        else if s1.ends_with("eight") {
            sum += 8;
        }
        else if s1.ends_with("nine") {
            sum += 9;
        }
        else {
            unreachable!();
        }
    }
    sum
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
        println!("{:?}", input);
        //assert_eq!(142, task_one(input.clone()));
        assert_eq!(281, task_two(input));
    }
}
