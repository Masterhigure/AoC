#![allow(unused)]
use std::fs;
use std::collections::{HashMap, HashSet};

type Input = Vec<Vec<char>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    let mut goal = (0,0);
    let mut pos = (0,0,'>');
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 'S' {
                pos = (i, j, '>');
            }
            if input[i][j] == 'E' {
                goal = (i, j);
            }
        }
    }
    input[goal.0][goal.1] = '.';
    let mut todo = HashMap::new();
    let mut visited = HashSet::new();
    todo.insert(0, vec![pos]);
    for score in 0.. {
        let v = todo.get(&score).unwrap_or(&Vec::new()).clone();
        for p in v {
            if (p.0, p.1) == goal {
                return score;
            }
            match p.2 {
                '>' => {
                    if input[p.0][p.1 + 1] == '.' && visited.insert((p.0, p.1 + 1, p.2)){
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0, p.1+1, p.2));
                    }
                },
                'v' => {
                    if input[p.0 + 1][p.1] == '.'  && visited.insert((p.0 + 1, p.1, p.2)){
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0 + 1, p.1, p.2));
                    }
                },
                '<' => {
                    if input[p.0][p.1 - 1] == '.'  && visited.insert((p.0, p.1 - 1, p.2)){
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0, p.1-1, p.2));
                    }
                },
                '^' => {
                    if input[p.0 - 1][p.1] == '.'  && visited.insert((p.0 - 1, p.1, p.2)){
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0 - 1, p.1, p.2));
                    }
                },
                _ => unreachable!(),
            }
            for d in ['>', 'v', '^', '<'].into_iter() {
                if d != p.2 && visited.insert((p.0, p.1, d)) {
                    todo.entry(score + 1000).or_insert(Vec::new()).push((p.0, p.1, d));
                }
            }
        }
    }
    0
}

fn task_two(mut input: Input) -> usize {
    let mut goal = (0,0);
    let mut pos = (0,0,'>', Vec::new());
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 'S' {
                pos = (i, j, '>', vec![(i, j)]);
            }
            if input[i][j] == 'E' {
                goal = (i, j);
            }
        }
    }
    input[goal.0][goal.1] = '.';
    let mut todo = HashMap::new();
    let mut visited = HashMap::new();
    todo.insert(0, vec![pos]);
    for score in 0.. {
        let v = todo.get(&score).unwrap_or(&Vec::new()).clone();
        let mut seats = HashSet::new();
        for mut p in v {
            if (p.0, p.1) == goal {
                for &c in &p.3 {
                    seats.insert(c);
                }
                continue;
            }
            match p.2 {
                '>' => {
                    if input[p.0][p.1 + 1] == '.' && *visited.entry(&(p.0, p.1 + 1, p.2)).or_insert(score + 1) < score {
                        p.3.push((p.0, p.1+1));
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0, p.1+1, p.2, p.3.clone()));
                        p.3.pop();
                    }
                },
                'v' => {
                    if input[p.0 + 1][p.1] == '.'  && *visited.entry(&(p.0 + 1, p.1, p.2)).or_insert(score + 1) < score {
                        p.3.push((p.0+1, p.1));
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0 + 1, p.1, p.2, p.3.clone()));
                        p.3.pop();
                    }
                },
                '<' => {
                    if input[p.0][p.1 - 1] == '.'  && *visited.entry(&(p.0, p.1 - 1, p.2)).or_insert(score + 1) < score {
                        p.3.push((p.0, p.1 - 1));
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0, p.1-1, p.2, p.3.clone()));
                        p.3.pop();
                    }
                },
                '^' => {
                    if input[p.0 - 1][p.1] == '.'  && *visited.entry(&(p.0 - 1, p.1, p.2)).or_insert(score + 1) < score {
                        p.3.push((p.0-1, p.1));
                        todo.entry(score + 1).or_insert(Vec::new()).push((p.0 - 1, p.1, p.2, p.3.clone()));
                        p.3.pop();
                    }
                },
                _ => unreachable!(),
            }
            for d in ['>', 'v', '^', '<'].into_iter() {
                let s = visited.entry(&(p.0, p.1, d)).or_insert(score + 1000);
                if d != p.2 && *s < score {
                    todo.entry(score + 1000).or_insert(Vec::new()).push((p.0, p.1, d, p.3.clone()));
                }
            }
        }
        if !seats.is_empty() {
            for i in 0..input.len() {
                for j in 0..input[0].len() {
                    if input[i][j] == '#' {
                        print!("#");
                    } else if seats.contains(&(i, j)) {
                        print!("O");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            return seats.len();
        }
    }
    0
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
        assert_eq!(7036, task_one(input.clone()));
        assert_eq!(45, task_two(input));
    }
}
