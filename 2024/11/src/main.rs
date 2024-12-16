#![allow(unused)]
#![feature(linked_list_cursors)]
use std::collections::{LinkedList, HashMap};
use std::fs;

type Input = LinkedList<usize>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    for _ in 0..25 {
        let mut c = input.cursor_front_mut();
        while let Some(v) = c.current() {
            if *v == 0 {
                *v = 1;
            } else if v.ilog10() % 2 == 1 {
                let p = 10_usize.pow(v.ilog10() / 2 + 1);
                let n = *v % p;
                *v = *v / p;
                c.insert_after(n);
                c.move_next();
            } else {
                *v *= 2024;
            }
            c.move_next();
        }
    }
    input.len()
}

fn task_two(mut input: Input) -> usize {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    for s in input {
        if let Some(old) = stones.insert(s, 1) {
            let l = stones.get_mut(&s).unwrap();
            *l = *l + old;
        }
    }
    for _ in 0..75 {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();
        for (&s, &n) in &stones {
            if s == 0 {
                new_stones.entry(1).and_modify(|e| *e += n).or_insert(n);
            } else if s.ilog10()%2 == 1 {
                let p = 10_usize.pow(s.ilog10() / 2 + 1);
                let f = s / p;
                let b = s % p;
                new_stones.entry(f).and_modify(|e| *e += n).or_insert(n);
                new_stones.entry(b).and_modify(|e| *e += n).or_insert(n);
            } else {
                new_stones.entry(s*2024).and_modify(|e| *e += n).or_insert(n);
            }
        }
        stones = new_stones;
    }
    stones.values().sum::<usize>()
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
        assert_eq!(55312, task_one(input.clone()));
        //assert_eq!(55312, task_two(input));
    }
}
