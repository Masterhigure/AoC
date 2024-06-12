#![allow(unused)]
use std::fs;

use std::str::FromStr;

#[derive(Debug, Default, Clone, Hash)]
struct Ticket {
    wins: Vec<i32>,
    nums: Vec<i32>,
}

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w, n) = s.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let wins = w.split(' ')
            .filter(|c| !c.is_empty())
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();
        let nums = n.split(' ')
            .filter(|c| !c.is_empty())
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();
        Ok(Self {
            wins,
            nums
        })
    }
}

type Input = Vec<Ticket>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input) -> i32 {
    let mut s = 0;
    for t in input {
        let mut score = 0;
        for n in t.nums {
            if t.wins.contains(&n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        s += score;
    }
    s
}

fn task_two(mut input: Input) -> i32 {
    let mut copies = std::iter::repeat(1).take(input.len()).collect::<Vec<_>>();
    for (x, t) in input.iter().enumerate() {
        let mut score = 0;
        for n in &t.nums {
            if t.wins.contains(&n) {
                score += 1;
            }
        }
        for c in (x+1)..(copies.len().min(score as usize + x + 1)) {
            copies[c] += copies[x];
        }
    }
    copies.iter().sum::<i32>()
}

fn recursive_solve(full_list: &Input, given_list: &[Ticket], index: usize) -> i32 {
    let mut s = 0;
    for (x, c) in given_list.iter().enumerate() {
        s += 1;
        let mut score = 0;
        for n in &c.nums {
            if c.wins.contains(&n) {
                score += 1;
            }
        }
        s += recursive_solve(&full_list, &full_list[(index+x+1)..(index+x+score+1)], index + x + 1)
    }
    s
}

fn main() {
    let input = read_input("input.txt");
    //println!("Task one: {}", task_one(input.clone()));
    println!("Task two_recursive: {}", recursive_solve(&input, &input[..], 0));
    //println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(13, task_one(input.clone()));
        assert_eq!(30, task_two(input));
    }
}
