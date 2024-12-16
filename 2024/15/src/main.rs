#![allow(unused)]
use std::collections::HashSet;
use std::fs;

type Input = (Vec<Vec<char>>, Vec<char>);

fn read_input(filename: &str) -> Input {
    let binding = fs::read_to_string(filename).unwrap();
    let (s1, s2) = binding.split_once("\n\n").unwrap();
    let maze = s1.split('\n').map(|s| s.chars().collect()).collect();
    let commands = s2.chars().filter(|&c| c != '\n').collect();
    (maze, commands)
}

fn task_one(mut input: Input) -> usize {
    let mut pos = (0, 0);
    let mut maze = input.0;
    let cmds = input.1;
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == '@' {
                pos = (i as i32, j as i32);
                maze[i][j] = '.';
            }
        }
    }
    for c in cmds {
        let (x, y) = match c {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => unreachable!(),
        };
        let mut scan = (pos.0 + x, pos.1 + y);
        while maze[scan.0 as usize][scan.1 as usize] == 'O' {
            scan = (scan.0 + x, scan.1 + y);
        }
        if maze[scan.0 as usize][scan.1 as usize] == '.' {
            maze[scan.0 as usize][scan.1 as usize] = 'O';
            pos = (pos.0 + x, pos.1 + y);
            maze[pos.0 as usize][pos.1 as usize] = '.';
        }
    }
    let mut s = 0;
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'O' {
                s += 100 * i + j;
            }
        }
    }
    s
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Hash)]
struct Eske {
    l: (i32, i32),
    r: (i32, i32),
}

fn task_two(mut input: Input) -> usize {
    let mut maze = input.0;
    let cmds = input.1;

    maze = maze
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| match c {
                    '#' => ['#', '#'].into_iter(),
                    '.' => ['.', '.'].into_iter(),
                    '@' => ['@', '.'].into_iter(),
                    'O' => ['[', ']'].into_iter(),
                    _ => unreachable!(),
                })
                .flatten()
                .collect()
        })
        .collect();
    let mut pos = (0, 0);
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == '@' {
                pos = (i as i32, j as i32);
                maze[i][j] = '.';
            }
        }
    }
    for c in cmds {
        let (x, y) = match c {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => unreachable!(),
        };
        if c == '<' || c == '>' {
            let mut scan = (pos.0 + x, pos.1 + y);
            let mut to_be_flipped = Vec::new();
            while ['[', ']'].contains(&maze[scan.0 as usize][scan.1 as usize]) {
                to_be_flipped.push(scan);
                to_be_flipped.push((scan.0+x, scan.1 + y));
                scan = (scan.0 + 2*x, scan.1 + 2*y);
            }
            if maze[scan.0 as usize][scan.1 as usize] == '.' {
                to_be_flipped.push(scan);
                for (m, n) in to_be_flipped {
                     if maze[m as usize][n as usize] == '[' {
                        maze[m as usize][n as usize] =']';
                    } else if maze[m as usize][n as usize] == ']' {
                        maze[m as usize][n as usize] ='[';
                    } else if maze[m as usize][n as usize] == '.' {
                        maze[m as usize][n as usize] =['[', ']'][((y+1) as usize)/2];
                    }
                }
                pos = (pos.0 + x, pos.1 + y);
                maze[pos.0 as usize][pos.1 as usize] = '.';
            }
        } else {
            if c == '^' {
                maze = maze.into_iter().rev().collect();
                pos = (maze.len() as i32 - pos.0 - 1, pos.1);
            }
            let mut to_be_checked = Vec::new();
            if maze[pos.0 as usize + 1][pos.1 as usize] == '[' {
                to_be_checked.push(Eske {
                    l: (pos.0 + 1, pos.1),
                    r: (pos.0 + 1, pos.1 + 1),
                });
            } else if maze[pos.0 as usize + 1][pos.1 as usize] == ']' {
                to_be_checked.push(Eske {
                    l: (pos.0 + 1, pos.1 - 1),
                    r: (pos.0 + 1, pos.1),
                });
            } else if maze[pos.0 as usize + 1][pos.1 as usize] == '#' {
                pos = (pos.0 - 1, pos.1);
            }
            let mut can_move = true;
            let mut k = 0;
            while k < to_be_checked.len() && can_move {
                let e = to_be_checked[k];
                for &t in [e.l, e.r].iter() {
                    if maze[t.0 as usize + 1][t.1 as usize] == '#' {
                        can_move = false;
                        break;
                    }
                    if maze[t.0 as usize + 1][t.1 as usize] == '[' {
                        to_be_checked.push(Eske{l: (t.0 + 1, t.1), r: (t.0 + 1, t.1+1)});
                    }
                    if maze[t.0 as usize + 1][t.1 as usize] == ']' {
                        to_be_checked.push(Eske{l: (t.0 + 1, t.1-1), r: (t.0+1, t.1)});
                    }
                }
                k += 1;
            }
            if can_move {
                for e in &to_be_checked {
                    maze[e.l.0 as usize][e.l.1 as usize] = '.';
                    maze[e.r.0 as usize][e.r.1 as usize] = '.'; 
                }
                for e in to_be_checked {
                    maze[e.l.0 as usize + 1][e.l.1 as usize] = '[';
                    maze[e.r.0 as usize + 1][e.r.1 as usize] = ']';
                }
                pos = (pos.0 + 1, pos.1);
            }
            if c == '^' {
                maze = maze.into_iter().rev().collect();
                pos = (maze.len() as i32 - pos.0 - 1, pos.1);
            }
        }
    }
    let mut s = 0;
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == '[' {
                s += 100 * i + j;
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
        //assert_eq!(10092, task_one(input.clone()));
        assert_eq!(9021, task_two(input));
    }
}
