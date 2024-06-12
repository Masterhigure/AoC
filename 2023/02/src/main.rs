#![allow(unused)]
use std::fs;
use std::str::FromStr;

#[derive(Default, Debug, Copy, Clone)]
struct Selection {
    red: i32,
    blue: i32,
    green: i32,
}

impl FromStr for Selection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut selection = Self::default();
        s.split(", ")
            .map(|t| t.split_once(' ').unwrap())
            .for_each(|(n, c)| {
                match c {
                    "red" => selection.red = n.parse::<i32>().unwrap(),
                    "blue" => selection.blue = n.parse::<i32>().unwrap(),
                    "green" => selection.green = n.parse::<i32>().unwrap(),
                    _ => unreachable!(),
                }
            });
        Ok(selection)
    }
}

type Input = Vec<Vec<Selection>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| 
            s.split_once(": ")
            .unwrap().1
            .split("; ")
            .map(|t| t.parse().unwrap())
            .collect())
        .collect()
}

fn task_one(mut i: Input) -> usize {
    i.iter().enumerate()
        .map(|(n, selections)| {
            let mut within_limits = true;
            selections.iter().for_each(|&s| {
                if s.red > 12 || s.green > 13 || s.blue > 14 {
                    within_limits = false;
                }
            });
            if within_limits {
                n + 1
            }
            else {
                0
            }
        })
        .sum()
}

fn task_two(mut i: Input) -> i32 {
    i.iter()
        .map(|selections| {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            selections.iter().for_each(|&s| {
                red = red.max(s.red);
                blue = blue.max(s.blue);
                green = green.max(s.green);
            });
            red*blue*green
        })
        .sum()
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
        assert_eq!(8, task_one(input.clone()));
        assert_eq!(2286, task_two(input));
    }
}
