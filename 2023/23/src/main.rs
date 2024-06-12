#![allow(unused)]
use std::fs;
use std::collections::{HashSet, HashMap};

type Input = Vec<Vec<char>>;

fn read_input(filename: &str) -> Input {
    let mut v = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    v[0][1] = '#';
    v
}

fn task_one(mut input: Input) -> usize {
    let mut pos = (1, 1);
    let mut finished = Vec::new();
    let mut in_progress = vec![vec![(1, 1)]];
    while !in_progress.is_empty() {
        let path = in_progress.pop().unwrap();
        let pos = path[path.len() - 1];
        let mut neighs = [
            (pos.0-1, pos.1, '^'),
            (pos.0, pos.1+1, '>'),
            (pos.0+1, pos.1, 'v'),
            (pos.0, pos.1-1, '<'),
        ];
        for (nx, ny, d) in neighs {
            let mut p = path.clone();
            if !p.contains(&(nx, ny)) && (input[nx][ny] == '.' || input[nx][ny] == d) {
                p.push((nx, ny));
                if nx + 1 == input.len() {
                    finished.push(p);
                } else {
                    in_progress.push(p);
                }
            }
        }
    }
    finished.iter().map(|v| v.len()).max().unwrap()
}

fn task_two(mut input: Input) -> usize {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] != '#' {
                input[i][j] = '.';
            }
        }
    }
    let mx = input.len() - 1;
    let my = input[0].len() - 1;
    input[mx][my - 1] = '#';

    let mut vertices = vec![(1, 1), (input.len() - 2, input[0].len() - 2)];
    for i in 1..(input.len() - 1) {
        for j in 1..(input[0].len() - 1) {
            if input[i][j] == '.' {
                let mut c = 0;
                if input[i-1][j] == '.' {
                    c += 1;
                }
                if input[i+1][j] == '.' {
                    c += 1;
                }
                if input[i][j-1] == '.' {
                    c += 1;
                }
                if input[i][j+1] == '.' {
                    c += 1;
                }
                if c > 2 {
                    vertices.push((i, j));
                }
            }
        }
    }
    
    let mut distances: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..vertices.len() {
        let start = vertices[i];
        let starts = [
            (start.0-1, start.1),
            (start.0, start.1+1),
            (start.0+1, start.1),
            (start.0, start.1-1),
        ];
        for mut pos in starts {
            if input[pos.0][pos.1] == '#' {
                continue;
            }
            let mut dist = 1;
            let mut path = HashSet::new();
            path.insert(pos);
            let mut prev_pos = start;
            loop {
                let mut neighs = [
                    (pos.0-1, pos.1),
                    (pos.0, pos.1+1),
                    (pos.0+1, pos.1),
                    (pos.0, pos.1-1),
                ];
                for neigh in neighs {
                    if neigh == prev_pos || input[neigh.0][neigh.1] == '#' {
                        continue;
                    }
                    prev_pos = pos;
                    pos = neigh;
                    dist += 1;
                    path.insert(pos);
                    break;
                }
                if let Some(j) = vertices.iter().position(|v| *v == pos) {
                    if let Some(ref mut v) = distances.get_mut(&i) {
                        v.push((j, path.len()));
                    } else {
                        distances.insert(i, vec![(j, path.len())]);
                    }
                    break;
                }
            }
        }
    }
    
    let mut pos = 0;
    let mut finished = Vec::new();
    let mut in_progress = vec![vec![(0, 1)]];
    while !in_progress.is_empty() {
        let path = in_progress.pop().unwrap();
        let (pos, dist) = path[path.len() - 1];
        for &(j, d) in distances.get(&pos).unwrap() {
            let mut p = path.clone();
            if p.iter().all(|(k, _)| *k != j) {
                p.push((j, d + dist));
                if j == 1 {
                    finished.push(p);
                } else {
                    in_progress.push(p);
                }
            }
        }
    }
    finished.iter().map(|v| v[v.len() - 1].1 + 1).max().unwrap()
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
        assert_eq!(94, task_one(input.clone()));
        assert_eq!(154, task_two(input));
    }
}
