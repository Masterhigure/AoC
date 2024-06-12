#![allow(unused)]
#![feature(isqrt)]
use std::fs;

type Input = Vec<(i64, i64)>;

fn read_input(filename: &str) -> Input {
    let binding = fs::read_to_string(filename)
        .unwrap();
    let (t, d) = binding.split_once('\n')
        .unwrap();
    let t: Vec<_> = t.split_once(':').unwrap().1
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();
    let d: Vec<_> = d.split_once(':').unwrap().1
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();
    t.into_iter().zip(d.into_iter()).collect()
}

fn task_one(mut input: Input) -> i64 {
    let mut s = 1;
    for (time, distance) in input {
        let mut n = 0;
        for t in 0..=time {
            if t*(time - t) > distance {
                n += 1;
            }
        }
        s *= n;
    }
    s
}

fn task_two(mut input: Input) -> i64 {
    let mut time = 0;
    let mut dist = 0;
    for (t, d) in input {
        time = time*10_i64.pow(t.ilog10() + 1) + t;
        dist = dist*10_i64.pow(d.ilog10() + 1) + d;
    }
    let mut min_time = (time-(time*time - 4*dist).isqrt());
    time - min_time
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
        assert_eq!(288, task_one(input.clone()));
        assert_eq!(71503, task_two(input));
    }
}
