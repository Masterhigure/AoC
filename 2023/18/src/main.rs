#![allow(unused)]
use std::fs;
use std::str::FromStr;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct Command {
    d: i32,
    l: usize,
    c: (i32, i32, i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let d = match parts[0] {
            "U" => 0,
            "R" => 1,
            "D" => 2,
            "L" => 3,
            _ => unreachable!(),
        };
        let l = parts[1].parse().unwrap();
        let hex = i32::from_str_radix(&parts[2][2..8], 16).unwrap();
        let c = (hex/65536, (hex/256)%256, hex%256);
        Ok(Command{d, l, c})
    }
}

type Input = Vec<Command>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    let mut x = 184;
    let mut y = 130;
    let mut grid = Vec::new();
    for _ in 0..(161+185) {
        let row = vec!['.'; 200+131];
        grid.push(row);
    }
    grid[x][y] = '#';
    for r in input {
        match r.d {
            0 => {
                for i in 0..r.l {
                    x -= 1;
                    grid[x][y] = '#';
                }
            },
            1 => {
                for i in 0..r.l {
                    y += 1;
                    grid[x][y] = '#';
                }
            }
            2 => {
                for i in 0..r.l {
                    x += 1;
                    grid[x][y] = '#';
                }
            }
            3 => {
                for i in 0..r.l {
                    y -= 1;
                    grid[x][y] = '#';
                }
            },
            _ => unreachable!(),
        }
    }
    'outer: for i in 0..grid.len() {
        for j in 1..(grid[i].len()-1) {
            if grid[i][j] == '#' {
                if grid[i][j-1] == '.' && grid[i][j+1] == '.' {
                    x = i;
                    y = j + 1;
                    break 'outer;
                }
                break;
            }
        }
    }
    let mut to_fill = vec![(x, y)];
    while !to_fill.is_empty() {
        (x, y) = to_fill.pop().unwrap();
        grid[x][y] = '#';
        let to_add = [
            (x-1, y-1),
            (x-1, y),
            (x-1, y+1),
            (x, y-1),
            (x, y+1),
            (x+1, y+1),
            (x+1, y),
            (x+1, y-1),
        ];
        for (i, j) in to_add {
            if !to_fill.contains(&(i, j)) && grid[i][j] == '.' {
                to_fill.push((i, j));
            }
        }

    }
    grid.iter().map(|r| r.iter()).flatten().filter(|&&c| c == '#').count()
}

fn task_two(mut input: Input) -> isize {
    let mut vert_corr: isize = 0;
    let mut j_corr: isize = 0;
    let mut seven_corr: isize = 0;
    let mut y = 0;
    let mut prev_d = input[input.len() - 1].c.2%16;
    let mut circ = 0;

    let mut area: isize = 0;
    
    for cmd in input {
        let l = (cmd.c.0*16*256 + cmd.c.1*16 + cmd.c.2/16) as isize;
        circ += l;
        match cmd.c.2%16 {
            0 => {
                if prev_d == 1 {
                    j_corr += y;
                }
                if prev_d == 3 {
                    seven_corr -= y;
                }
                area += y*l;
            },
            1 => {
                if prev_d == 0 {
                    j_corr -= y;
                }
                if prev_d == 2 {
                    seven_corr -= y;
                }
                y += l;
            },
            2 => {
                if prev_d == 3 {
                    j_corr -= y;
                }
                if prev_d == 1 {
                    seven_corr += y;
                }
                area -= y*l;
                vert_corr += l;
            },
            3 => {
                if prev_d == 2 {
                    j_corr += y;
                }
                if prev_d == 0 {
                    seven_corr += y;
                }
                y -= l;
            },
            _ => unreachable!(),
        }
        prev_d = cmd.c.2%16;
    }
    dbg!(&area, &vert_corr, &seven_corr, &j_corr, &circ);
    if area < 0 {
        -area - vert_corr - seven_corr + circ + 1
    } else {
        // Off-by-one here too? Dunno, haven't checked.
        area - vert_corr - j_corr + circ
    }
    
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
        assert_eq!(62, task_one(input.clone()));
        assert_eq!(952408144115, task_two(input));
    }
}
