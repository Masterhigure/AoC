#![allow(unused)]
use std::fs;

type Grid = Vec<Vec<char>>;

type Input = Vec<Grid>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.split('\n').map(|r| r.chars().collect()).collect())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    let mut s = 0;
    'outer: for g in input {
        // Horizontal axis
        for i in 0..(g.len() - 1) {
            let ub = if i*2 + 1 < g.len() {
                i
            } else {
                g.len() - i - 2
            };
            if (0..=ub).all(|j| g[i+j+1] == g[i-j]) {
                s += 100*i + 100;
                continue 'outer;
            }
        }

        // Vertical axis
        for i in 0..(g[0].len() - 1) {
            let ub = if i*2 + 1 < g[0].len() {
                i
            } else {
                g[0].len() - i - 2
            };
            if (0..=ub).all(|j| {
                let col_j = (0..g.len()).map(|k| g[k][i+j+1]);
                let col_ij = (0..g.len()).map(|k| g[k][i-j]);
                col_j.zip(col_ij).all(|(x, y)| x == y)
            }) {
                s += i + 1;
                continue 'outer;
            }
        }
    }
    s
}

#[derive(PartialEq, Eq)]
enum Axis {
    Row(usize),
    Col(usize),
}

fn task_two(mut input: Input) -> usize {
    let mut s = 0;
    'outer: for mut g in input {
        let mut a = Axis::Row(0);
        'lokke: loop {
            // Horizontal axis
            for i in 0..(g.len() - 1) {
                let ub = if i*2 + 1 < g.len() {
                    i
                } else {
                    g.len() - i - 2
                };
                if (0..=ub).all(|j| g[i+j+1] == g[i-j]) {
                    a = Axis::Row(i);
                    break 'lokke;
                }
            }

            // Vertical axis
            for i in 0..(g[0].len() - 1) {
                let ub = if i*2 + 1 < g[0].len() {
                    i
                } else {
                    g[0].len() - i - 2
                };
                if (0..=ub).all(|j| {
                    let col_j = (0..g.len()).map(|k| g[k][i+j+1]);
                    let col_ij = (0..g.len()).map(|k| g[k][i-j]);
                    col_j.zip(col_ij).all(|(x, y)| x == y)
                }) {
                    a = Axis::Col(i);
                    break 'lokke;
                }
            }
            unreachable!()
        }
        for m in 0..g.len() {
            for n in 0..g[0].len() {
                g[m][n] = if g[m][n] == '#' {'.'} else {'#'};
                for i in 0..(g.len() - 1) {
                    let ub = if i*2 + 1 < g.len() {
                        i
                    } else {
                        g.len() - i - 2
                    };
                    if (0..=ub).all(|j| g[i+j+1] == g[i-j]) {
                        if a != Axis::Row(i) {
                            s += 100*i + 100;
                            continue 'outer;
                        }
                    }
                }

                // Vertical axis
                for i in 0..(g[0].len() - 1) {
                    let ub = if i*2 + 1 < g[0].len() {
                        i
                    } else {
                        g[0].len() - i - 2
                    };
                    if (0..=ub).all(|j| {
                        let col_j = (0..g.len()).map(|k| g[k][i+j+1]);
                        let col_ij = (0..g.len()).map(|k| g[k][i-j]);
                        col_j.zip(col_ij).all(|(x, y)| x == y)
                    }) {
                        if a != Axis::Col(i) {
                            s += i + 1;
                            continue 'outer;
                        }
                    }
                }
                g[m][n] = if g[m][n] == '#' {'.'} else {'#'};
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
        assert_eq!(405, task_one(input.clone()));
        assert_eq!(400, task_two(input));
    }
}
