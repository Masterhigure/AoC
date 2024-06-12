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

fn task_one(mut input: Input) -> u32 {
    let mut s = 0;
    'rows: for i in 0..input.len() {
        let mut j = 0;
        while j < input[0].len() {
            while !input[i][j].is_ascii_digit() {
                j += 1;
                if j >= input[0].len() {
                    continue 'rows;
                }
            }
            let mut k = j;
            while input[i][k].is_ascii_digit() && k < input[0].len() - 1 {
                k += 1;
            }
            if !input[i][k].is_ascii_digit() {
                k -= 1;
            }
            let n = input[i][j..=k].iter()
                .fold(0, |acc, c| acc*10 + c.to_digit(10).unwrap());
            let mut neighs = vec![];
            if i > 0 {
                neighs.extend_from_slice(&input[i-1][j..=k]);
                if j > 0 {
                    neighs.push(input[i-1][j-1]);
                }
                if k + 1 < input.len() {
                    neighs.push(input[i-1][k+1]);
                }
            }
            if j > 0 {
                neighs.push(input[i][j-1]);
            }
            if k + 1 < input.len() {
                neighs.push(input[i][k+1]);
            }
            if i + 1 < input.len() {
                neighs.extend_from_slice(&input[i+1][j..=k]);
                if j > 0 {
                    neighs.push(input[i+1][j-1]);
                }
                if k + 1 < input.len() {
                    neighs.push(input[i+1][k+1]);
                }
            }
            if !neighs.iter().all(|&c| c == '.' || c.is_ascii_digit()) {
                s += n;
            }
            j = k+2;
        }
    }
    s
}

fn locate_number(input: &Input, i: usize, j: usize) -> (u32, usize) {
    let mut m = j;
    let mut n = j;
    while m > 0 && input[i][m].is_ascii_digit() {
        m -= 1;
    }
    if !input[i][m].is_ascii_digit() {
        m += 1;
    }
    while n + 1 < input.len() && input[i][n].is_ascii_digit() {
        n += 1;
    }
    if !input[i][n].is_ascii_digit() {
        n -= 1;
    }
    (input[i][m..=n].iter().fold(0, |acc, c| acc*10 + c.to_digit(10).unwrap()), m)
}

fn task_two(mut input: Input) -> u32 {
    let mut s = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '*' {
                let mut neighs = HashSet::new();
                if i > 0 {
                    if j > 0 && input[i-1][j-1].is_ascii_digit() {
                        neighs.insert(locate_number(&input, i-1, j-1));
                    }
                    if input[i-1][j].is_ascii_digit() {
                        neighs.insert(locate_number(&input, i-1, j));
                    }
                    if j + 1 < input[0].len() && input[i-1][j+1].is_ascii_digit() {
                        neighs.insert(locate_number(&input, i-1, j+1));
                    }
                }
                if j > 0 && input[i][j-1].is_ascii_digit() {
                    neighs.insert(locate_number(&input, i, j-1));
                }
                if j + 1 < input[0].len() && input[i][j+1].is_ascii_digit() {
                    neighs.insert(locate_number(&input, i, j+1));
                }
                if i + 1 < input.len() {
                    if j > 0 && input[i+1][j-1].is_ascii_digit() {
                        neighs.insert(locate_number(&input, i+1, j-1));
                    }
                    if input[i+1][j].is_ascii_digit() {
                        neighs.insert(locate_number(&input, i+1, j));
                    }
                    if j + 1 < input[0].len() && input[i+1][j+1].is_ascii_digit() {
                        neighs.insert(locate_number(&input, i+1, j+1));
                    }
                }
                if neighs.len() == 2 {
                    s += neighs.iter().map(|(n, _)| n).product::<u32>();
                }
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
        assert_eq!(4361, task_one(input.clone()));
        assert_eq!(467835, task_two(input));
    }
}
