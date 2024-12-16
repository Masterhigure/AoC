#![allow(unused)]
use std::fs;
use std::collections::HashSet;

type Input = Vec<Vec<u32>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_ends(mut ends: &mut HashSet<(usize, usize)>, input: &Input, i: usize, j: usize) {
    let h = input[i][j];
    if h == 9 {
        ends.insert((i, j));
        return;
    }
    if i > 0 && input[i-1][j] == h+1 {
        find_ends(&mut ends, &input, i-1, j);
    }
    if i < input.len() - 1 && input[i+1][j] == h+1 {
        find_ends(&mut ends, &input, i+1, j);
    }
    if j > 0 && input[i][j-1] == h+1 {
        find_ends(&mut ends, &input, i, j-1);
    }
    if j < input[0].len() - 1 && input[i][j+1] == h+1 {
        find_ends(&mut ends, &input, i, j+1);
    }
}

fn find_trails(mut trails: &mut HashSet<Vec<(usize, usize)>>,
               input: &Input, 
               mut path: &mut Vec<(usize, usize)>, 
               i: usize, 
               j: usize) {
    let h = input[i][j];
    if h == 9 {
        trails.insert(path.clone());
        return;
    }
    if i > 0 && input[i-1][j] == h+1 {
        path.push((i-1, j));
        find_trails(&mut trails, &input, path, i-1, j);
        path.pop();
    }
    if i < input.len() - 1 && input[i+1][j] == h+1 {
        path.push((i+1, j));
        find_trails(&mut trails, &input, path, i+1, j);
        path.pop();
    }
    if j > 0 && input[i][j-1] == h+1 {
        path.push((i, j-1));
        find_trails(&mut trails, &input, path, i, j-1);
        path.pop();
    }
    if j < input[0].len() - 1 && input[i][j+1] == h+1 {
        path.push((i, j+1));
        find_trails(&mut trails, &input, path, i, j+1);
        path.pop();
    }
}

fn task_one(mut input: Input) -> usize {
    let mut s = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 0 {
                let mut ends = HashSet::new();
                find_ends(&mut ends, &input, i, j);
                s += ends.len();
            }
        }
    }
    s
}

fn task_two(mut input: Input) -> usize {
    let mut s = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 0 {
                let mut trails = HashSet::new();
                let mut path = vec![(i, j)];
                find_trails(&mut trails, &input, &mut path, i, j);
                s += trails.len();
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
        assert_eq!(36, task_one(input.clone()));
        assert_eq!(81, task_two(input));
    }
}
