#![allow(unused)]
use std::fs;
use std::str::FromStr;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(|c: char| ['=', ',', ' '].contains(&c));
        i.next();
        let p = (i.next().unwrap().parse().unwrap(), i.next().unwrap().parse().unwrap());
        i.next();
        let v = (i.next().unwrap().parse().unwrap(), i.next().unwrap().parse().unwrap());
        Ok(Self{p, v})
    }
}

type Input = Vec<Robot>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input, w: i32, h: i32) -> i32 {
    for _ in 0..100 {
        for mut r in input.iter_mut() {
            r.p.0 = (r.p.0 + r.v.0 + w)%w;
            r.p.1 = (r.p.1 + r.v.1 + h)%h;
        }
    }
    let mut quads = [0,0,0,0];
    for r in &input {
        if r.p.0 < w/2 && r.p.1 < h/2 {
            quads[0] += 1;
        }
        if r.p.0 > w/2 && r.p.1 < h/2 {
            quads[1] += 1;
        }
        if r.p.0 > w/2 && r.p.1 > h/2 {
            quads[2] += 1;
        }
        if r.p.0 < w/2 && r.p.1 > h/2 {
            quads[3] += 1;
        }
    }
    quads.iter().product::<i32>()
}

fn task_two(mut input: Input) -> i32 {
    let mut s = 0;
    'outer: loop {
        let mut image = [[0_i32; 103]; 101];
        for mut r in input.iter_mut() {
            r.p.0 = (r.p.0 + r.v.0 + 101)%101;
            r.p.1 = (r.p.1 + r.v.1 + 103)%103;
            image[r.p.0 as usize][r.p.1 as usize] = 1;
        }
        s += 1;
        for i in 0..101 {
            for j in 0..(103 - 10) {
                if image[i][j..(j+10)] == [1, 1, 1, 1, 1, 1, 1, 1, 1, 1] {
                    break 'outer;
                }
            }
        }
    }

    s
}

fn main() {
    let input = read_input("input.txt");
    println!("Task one: {}", task_one(input.clone(), 101, 103));
    println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(12, task_one(input.clone(), 11, 7));
        //assert_eq!(0, task_two(input));
    }
}
