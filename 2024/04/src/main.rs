#![allow(unused)]
use std::fs;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect()
}

fn task_one(mut input: Input) -> i32 {
    let mut s = 0;
    let h = input.len();
    let w = input[0].len();
    for i in 0..h {
        for j in 0..w {
            if input[i][j] != 'X' {
                continue;
            }
            if j < w-3 {
                if input[i][j+1] == 'M' && 
                   input[i][j+2] == 'A' && 
                   input[i][j+3] == 'S' {
                    s += 1;
                }
            }
            if j > 2 {
                if input[i][j-1] == 'M' && 
                   input[i][j-2] == 'A' && 
                   input[i][j-3] == 'S' {
                    s += 1;
                }
            }
            if i < h-3 {
                if input[i+1][j] == 'M' && 
                   input[i+2][j] == 'A' && 
                   input[i+3][j] == 'S' {
                    s += 1;
                }
            }
            if i > 2 {
                if input[i-1][j] == 'M' && 
                   input[i-2][j] == 'A' && 
                   input[i-3][j] == 'S' {
                    s += 1;
                }
            }
            if j < w-3 && i < h-3 {
                if input[i+1][j+1] == 'M' && 
                   input[i+2][j+2] == 'A' && 
                   input[i+3][j+3] == 'S' {
                    s += 1;
                }
            }
            if j > 2 && i < h-3 {
                if input[i+1][j-1] == 'M' && 
                   input[i+2][j-2] == 'A' && 
                   input[i+3][j-3] == 'S' {
                    s += 1;
                }
            }
            if j < w-3 && i > 2 {
                if input[i-1][j+1] == 'M' && 
                   input[i-2][j+2] == 'A' && 
                   input[i-3][j+3] == 'S' {
                    s += 1;
                }
            }
            if j > 2 && i > 2 {
                if input[i-1][j-1] == 'M' && 
                   input[i-2][j-2] == 'A' && 
                   input[i-3][j-3] == 'S' {
                    s += 1;
                }
            }
        }
    }
    s
}

fn task_two(mut input: Input) -> i32 {
    let mut s = 0;
    let h = input.len();
    let w = input[0].len();
    let mas = HashSet::from(['M', 'S']);
    for i in 1..(h-1) {
        for j in 1..(w-1) {
            if input[i][j] != 'A' {
                continue;
            }
            let f = HashSet::from([input[i-1][j+1], input[i+1][j-1]]);
            let b = HashSet::from([input[i-1][j-1], input[i+1][j+1]]);
            if f == mas && b == mas {
                s += 1;
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
        assert_eq!(18, task_one(input.clone()));
        assert_eq!(9, task_two(input));
    }
}
