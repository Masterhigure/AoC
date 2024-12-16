#![allow(unused)]
use std::fs;

type Input = Vec<(i64, Vec<i64>)>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| {
            let (g, n) = s.split_once(": ").unwrap();
            (
                g.parse::<i64>().unwrap(),
                n.split(" ")
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect()
}

fn can_be_represented(goal: i64, mut nums: &mut Vec<i64>) -> bool {
    if nums.len() == 1 {
        return nums[0] == goal;
    }
    let last = nums.pop().unwrap();
    if goal % last == 0 && can_be_represented(goal / last, nums) {
        return true;
    }
    if goal >= last && can_be_represented(goal - last, nums) {
        return true;
    }
    nums.push(last);
    false
}

fn task_one(mut input: Input) -> i64 {
    let mut s = 0;
    for (goal, mut nums) in input {
        if can_be_represented(goal, &mut nums) {
            s += goal;
        }
    }
    s
}

fn can_be_represented_two(goal: i64, mut nums: &mut Vec<i64>) -> bool {
    if nums.len() == 1 {
        return nums[0] == goal;
    }
    let last = nums.pop().unwrap();
    if goal % last == 0 && can_be_represented_two(goal / last, nums) {
        return true;
    }
    if goal >= last && can_be_represented_two(goal - last, nums) {
        return true;
    }
    let pow = 10_i64.pow(last.ilog10() + 1);
    if (goal - last) % pow == 0 && can_be_represented_two(goal / pow, nums) {
        return true;
    }
    nums.push(last);
    false
}

fn task_two(mut input: Input) -> i64 {
    let mut s = 0;
    for (goal, mut nums) in input {
        if can_be_represented_two(goal, &mut nums) {
            s += goal;
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
        assert_eq!(3749, task_one(input.clone()));
        assert_eq!(11387, task_two(input));
    }
}
