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
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut beams: Vec<(usize, usize, usize)> = Vec::new();
    match input[0][0] {
        '.' | '-' => {
            visited.insert((0,0,1));
            beams.push((0,0,1));
        },
        '/' => {
            visited.insert((0,0,0));
            beams.push((0,0,0));
        },
        '\\' | '|' => {
            visited.insert((0,0,2));
            beams.push((0,0,2));
        }
        _ => unreachable!(),
    }
    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        if (beam.0 == 0 && beam.2 == 0)
            || (beam.0 + 1 == input.len() && beam.2 == 2)
            || (beam.1 == 0 && beam.2 == 3)
            || (beam.1 + 1 == input[0].len() && beam.2 == 1) {
            continue;
        }
        let next = match beam.2 {
            0 => (beam.0 - 1, beam.1),
            1 => (beam.0, beam.1 + 1),
            2 => (beam.0 + 1, beam.1),
            3 => (beam.0, beam.1 - 1),
            _ => unreachable!(),
        };
        match (input[next.0][next.1], beam.2) {
            ('.', _) | ('-', 1) | ('-', 3) | ('|', 0) | ('|', 2) => {
                if visited.insert((next.0, next.1, beam.2)) {
                    beams.push((next.0, next.1, beam.2));
                }
            },
            ('-', 0) | ('-', 2) => {
                if visited.insert((next.0, next.1, 1)) {
                    beams.push((next.0, next.1, 1));
                }
                if visited.insert((next.0, next.1, 3)) {
                    beams.push((next.0, next.1, 3));
                }
            },
            ('|', 1) | ('|', 3) => {
                if visited.insert((next.0, next.1, 0)) {
                    beams.push((next.0, next.1, 0));
                }
                if visited.insert((next.0, next.1, 2)) {
                    beams.push((next.0, next.1, 2));
                }
            },
            ('\\', d)=> {
                if visited.insert((next.0, next.1, 3-d)) {
                    beams.push((next.0, next.1, 3-d));
                }
            },
            ('/', d) => {
                if d % 2 == 0 {
                    if visited.insert((next.0, next.1, d + 1)) {
                        beams.push((next.0, next.1, d + 1));
                    }
                } else {
                    if visited.insert((next.0, next.1, d - 1)) {
                        beams.push((next.0, next.1, d - 1));
                    }
                };
            },
            _ => unreachable!(),
        }
    }
    let energized: HashSet<_> = visited.into_iter().map(|(a, b, _)| (a, b)).collect();
    energized.len()
}

fn task_two(mut input: Input) -> usize {
    let mut s = 0;
    let mut starts = Vec::new();
    for i in 0..input.len() {
        starts.push((i, 0, 1));
        starts.push((i, input[0].len() - 1, 3));
    }
    for i in 0..input[0].len() {
        starts.push((0, i, 2));
        starts.push((input.len() - 1, i, 0));
    }
    for start in starts {
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
        let mut beams: Vec<(usize, usize, usize)> = Vec::new();
        match (input[start.0][start.1], start.2) {
            ('.', _) | ('-', 1) | ('-', 3) | ('|', 0) | ('|', 2) => {
                visited.insert((start.0, start.1, start.2));
                beams.push((start.0, start.1, start.2));
            },
            ('-', 0) | ('-', 2) => {
                visited.insert((start.0, start.1, 1));
                beams.push((start.0, start.1, 1));
                visited.insert((start.0, start.1, 3));
                beams.push((start.0, start.1, 3));
            },
            ('|', 1) | ('|', 3) => {
                visited.insert((start.0, start.1, 0));
                beams.push((start.0, start.1, 0));
                visited.insert((start.0, start.1, 2));
                beams.push((start.0, start.1, 2));
            },
            ('\\', d)=> {
                visited.insert((start.0, start.1, 3-d));
                beams.push((start.0, start.1, 3-d));
            },
            ('/', d) => {
                if d % 2 == 0 {
                    visited.insert((start.0, start.1, d + 1));
                    beams.push((start.0, start.1, d + 1));
                } else {
                    visited.insert((start.0, start.1, d - 1));
                    beams.push((start.0, start.1, d - 1));
                };
            },
            _ => unreachable!(),
        }
        while !beams.is_empty() {
            let beam = beams.pop().unwrap();
            if (beam.0 == 0 && beam.2 == 0)
                || (beam.0 + 1 == input.len() && beam.2 == 2)
                || (beam.1 == 0 && beam.2 == 3)
                || (beam.1 + 1 == input[0].len() && beam.2 == 1) {
                continue;
            }
            let next = match beam.2 {
                0 => (beam.0 - 1, beam.1),
                1 => (beam.0, beam.1 + 1),
                2 => (beam.0 + 1, beam.1),
                3 => (beam.0, beam.1 - 1),
                _ => unreachable!(),
            };
            match (input[next.0][next.1], beam.2) {
                ('.', _) | ('-', 1) | ('-', 3) | ('|', 0) | ('|', 2) => {
                    if visited.insert((next.0, next.1, beam.2)) {
                        beams.push((next.0, next.1, beam.2));
                    }
                },
                ('-', 0) | ('-', 2) => {
                    if visited.insert((next.0, next.1, 1)) {
                        beams.push((next.0, next.1, 1));
                    }
                    if visited.insert((next.0, next.1, 3)) {
                        beams.push((next.0, next.1, 3));
                    }
                },
                ('|', 1) | ('|', 3) => {
                    if visited.insert((next.0, next.1, 0)) {
                        beams.push((next.0, next.1, 0));
                    }
                    if visited.insert((next.0, next.1, 2)) {
                        beams.push((next.0, next.1, 2));
                    }
                },
                ('\\', d)=> {
                    if visited.insert((next.0, next.1, 3-d)) {
                        beams.push((next.0, next.1, 3-d));
                    }
                },
                ('/', d) => {
                    if d % 2 == 0 {
                        if visited.insert((next.0, next.1, d + 1)) {
                            beams.push((next.0, next.1, d + 1));
                        }
                    } else {
                        if visited.insert((next.0, next.1, d - 1)) {
                            beams.push((next.0, next.1, d - 1));
                        }
                    };
                },
                _ => unreachable!(),
            }
        }
        let energized: HashSet<_> = visited.into_iter().map(|(a, b, _)| (a, b)).collect();
        let n = energized.len();
        s = s.max(n);
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
        assert_eq!(46, task_one(input.clone()));
        assert_eq!(51, task_two(input));
    }
}
