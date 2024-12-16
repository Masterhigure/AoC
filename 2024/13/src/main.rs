#![allow(unused)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
struct Machine{
    a: (i64, i64),
    b: (i64, i64),
    g: (i64, i64),
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split('\n').collect::<Vec<_>>();
        v[0] = v[0].split_once('+').unwrap().1;
        let a = v[0].split(", Y+").map(|s| s.parse().unwrap()).next_tuple().unwrap();
        v[1] = v[1].split_once('+').unwrap().1;
        let b = v[1].split(", Y+").map(|s| s.parse().unwrap()).next_tuple().unwrap();
        v[2] = v[2].split_once('=').unwrap().1;
        let g = v[2].split(", Y=").map(|s| s.parse().unwrap()).next_tuple().unwrap();
        Ok(Machine{a, b, g})
    }
}

type Input = Vec<Machine>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input) -> i64 {
    let mut s = 0;
    for m in input {
        let (ax, ay) = m.a;
        let (bx, by) = m.b;
        let (gx, gy) = m.g;
        let mut best = i64::MAX;
        for m in 0..=100 {
            for n in 0..=100 {
                if m*ax + n*bx == gx && m*ay + n*by == gy {
                    best = best.min(3*m + n);
                }
            }
        }
        if best < i64::MAX {
            s += best;
        }
    }
    s
}

fn gcd(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        return (x, 1, 0);
    }
    let (r, m, n) = gcd(y, x%y);
    return (r, n, m - (x/y)*n)
}

fn task_two(mut input: Input) -> i64 {
    let mut s = 0;
    for m in input {
        let (mut ax, mut ay) = m.a;
        let (mut bx, mut by) = m.b;
        let (mut gx, mut gy) = m.g;
        gx += 10_000_000_000_000;
        gy += 10_000_000_000_000;
        let (rx, mut mx, mut nx) = gcd(ax, bx);
        let (ry, mut my, mut ny) = gcd(ay, by);
        if gx % rx != 0 || gy % ry != 0 {
            continue;
        }
        let det = ax*by - bx*ay;
        assert!(det != 0);
        let nx = gx*by - gy*bx;
        let ny = -gx*ay + gy*ax;
        if nx % det != 0 || ny % det != 0 || nx/det < 0 || ny/det < 0 {
            continue;
        }
        s += 3*nx/det + ny/det;
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
        assert_eq!(480, task_one(input.clone()));
        assert_eq!(480, task_two(input));
    }
}
