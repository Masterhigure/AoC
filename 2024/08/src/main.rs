#![allow(unused)]
use std::fs;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub, Mul};

type Input = Vec<Vec<char>>;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<usize> for Point {
    type Output = Self;
    fn mul(self, other: usize) -> Self {
        Point {x: self.x * other as i32, y: self.y * other as i32}
    }
}

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    let mut ants: HashMap<char, HashSet<Point>> = HashMap::new();
    let mut nodes: HashSet<Point> = HashSet::new();
    for x in 0..input.len() as i32{
        for y in 0..input[0].len() as i32 {
            if input[x as usize][y as usize] != '.' {
                let c = Point {x, y};
                if let Some(v) = ants.get_mut(&input[x as usize][y as usize]) {
                    for p in v.iter() {
                        nodes.insert(*p + *p - c);
                        nodes.insert(c + c - *p);
                    }
                    v.insert(c);
                } else {
                    ants.insert(input[x as usize][y as usize], HashSet::from([c]));
                }
            }
        }
    }
    nodes.iter().filter(|p| {
        p.x >= 0 && p.x < input.len() as i32 && p.y >= 0 && p.y < input[0].len() as i32
    }).count()
}

fn task_two(mut input: Input) -> usize {
    let mut ants: HashMap<char, HashSet<Point>> = HashMap::new();
    let mut nodes: HashSet<Point> = HashSet::new();
    for x in 0..input.len() as i32{
        for y in 0..input[0].len() as i32 {
            if input[x as usize][y as usize] != '.' {
                let c = Point {x, y};
                if let Some(v) = ants.get_mut(&input[x as usize][y as usize]) {
                    for p in v.iter() {
                        let d = *p - c;
                        for i in 0..input.len().max(input[0].len()){
                            nodes.insert(c + d*i);
                            nodes.insert(c - d*i);
                        }
                    }
                    v.insert(c);
                } else {
                    ants.insert(input[x as usize][y as usize], HashSet::from([c]));
                }
            }
        }
    }
    nodes.iter().filter(|p| {
        p.x >= 0 && p.x < input.len() as i32 && p.y >= 0 && p.y < input[0].len() as i32
    }).count()
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
        assert_eq!(14, task_one(input.clone()));
        assert_eq!(0, task_two(input));
    }
}
