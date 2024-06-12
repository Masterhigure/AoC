#![allow(unused)]
use std::fs;

type Input = (Vec<(usize, usize)>, Vec<usize>, Vec<usize>);

fn read_input(filename: &str) -> Input {
    let rows = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(str::to_string)
        .collect::<Vec<String>>();
    let stars = rows.iter().enumerate()
        .map(|(e, r)| r.chars().enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(f, _)| (e, f)))
        .flatten().collect();
    let e_rows = rows.iter().enumerate()
        .filter(|(_, r)| r.chars().all(|c| c == '.'))
        .map(|(e, _)| e)
        .collect::<Vec<usize>>();
    let e_cols = (0..(rows[0].len()))
        .filter(|&i| {
            rows.iter().all(|s| s.chars().nth(i).unwrap() == '.')
        })
        .collect::<Vec<usize>>();
    (stars, e_rows, e_cols)
}

fn task_one(mut input: Input) -> usize {
    let mut s = 0;
    let stars = input.0;
    let e_rows = input.1;
    let e_cols = input.2;
    for i in 0..stars.len() {
        for j in 0..i {
            let s1 = stars[i];
            let s2 = stars[j];
            let xd = s1.0.abs_diff(s2.0);
            let yd = s1.1.abs_diff(s2.1);
            let x_ex = e_rows.iter().filter(|&r| ((s1.0.min(s2.0))..(s1.0.max(s2.0))).contains(r)).count();
            let y_ex = e_cols.iter().filter(|&r| ((s1.1.min(s2.1))..(s1.1.max(s2.1))).contains(r)).count();
            s += xd + yd + x_ex + y_ex;
        }
    }
    s
}

fn task_two(mut input: Input) -> usize {
    let mut s = 0;
    let stars = input.0;
    let e_rows = input.1;
    let e_cols = input.2;
    for i in 0..stars.len() {
        for j in 0..i {
            let s1 = stars[i];
            let s2 = stars[j];
            let xd = s1.0.abs_diff(s2.0);
            let yd = s1.1.abs_diff(s2.1);
            let x_ex = e_rows.iter().filter(|&r| ((s1.0.min(s2.0))..(s1.0.max(s2.0))).contains(r)).count();
            let y_ex = e_cols.iter().filter(|&r| ((s1.1.min(s2.1))..(s1.1.max(s2.1))).contains(r)).count();
            s += xd + yd + x_ex*999_999+ y_ex*999_999;
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
        assert_eq!(374, task_one(input.clone()));
        assert_eq!(0, task_two(input));
    }
}
