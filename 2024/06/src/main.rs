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

fn task_one(mut input: Input) -> usize {
    let mut positions = HashSet::new();
    let mut position = (0isize,0isize);
    'outer: for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] != '.' && input[i][j] != '#' {
                position = (i as isize,j as isize);
                break 'outer;
            }
        }
    }

    let mut dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1),(-1, 0)];
    let mut dir: usize = match input[position.0 as usize][position.1 as usize] {
        '>' => {0},
        'v' => {1},
        '<' => {2},
        '^' => {3},
        _ => {unreachable!()},
    };
    loop {
        positions.insert(position);
        if position.0 == 0 && dirs[dir].0 == -1 
            || position.1 == 0 && dirs[dir].1 == -1
            || position.0 as isize + dirs[dir].0 == input.len() as isize
            || position.1 as isize + dirs[dir].1 == input.len() as isize
        {
            break;
        }
        let next = (position.0 + dirs[dir].0, position.1 + dirs[dir].1);
        if input[next.0 as usize][next.1 as usize] == '#' {
            dir = (dir + 1)%4;
        } else {
            position = next;
        }
    }
    positions.len()
}

fn task_two(mut input: Input) -> i32 {
    let mut positions = HashSet::new();
    let mut position = (0isize,0isize);
    'outer: for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] != '.' && input[i][j] != '#' {
                position = (i as isize,j as isize);
                break 'outer;
            }
        }
    }
    let start_pos = position;

    let mut dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1),(-1, 0)];
    let mut dir: usize = match input[position.0 as usize][position.1 as usize] {
        '>' => {0},
        'v' => {1},
        '<' => {2},
        '^' => {3},
        _ => {unreachable!()},
    };
    let start_dir = dir;
    loop {
        positions.insert(position);
        if position.0 == 0 && dirs[dir].0 == -1 
            || position.1 == 0 && dirs[dir].1 == -1
            || position.0 as isize + dirs[dir].0 == input.len() as isize
            || position.1 as isize + dirs[dir].1 == input.len() as isize
        {
            break;
        }
        let next = (position.0 + dirs[dir].0, position.1 + dirs[dir].1);
        if input[next.0 as usize][next.1 as usize] == '#' {
            dir = (dir + 1)%4;
        } else {
            position = next;
        }
    }
    positions.remove(&start_pos);
    let mut s = 0;
    for obs in positions {
        input[obs.0 as usize][obs.1 as usize] = '#';
        position = start_pos;
        dir = start_dir;
        let mut turns = HashSet::new();
        loop {
            if position.0 == 0 && dirs[dir].0 == -1 
                || position.1 == 0 && dirs[dir].1 == -1
                || position.0 as isize + dirs[dir].0 == input.len() as isize
                || position.1 as isize + dirs[dir].1 == input.len() as isize
            {
                break;
            }
            let next = (position.0 + dirs[dir].0, position.1 + dirs[dir].1);
            if input[next.0 as usize][next.1 as usize] == '#' {
                if !turns.insert((position, dir)) {
                    s += 1;
                    break;
                }
                dir = (dir + 1)%4;
            } else {
                position = next;
            }
        }
        input[obs.0 as usize][obs.1 as usize] = '.';
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
        assert_eq!(41, task_one(input.clone()));
        assert_eq!(6, task_two(input));
    }
}
