#![allow(unused)]
use std::fs;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

type Input = (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>);

fn read_input(filename: &str) -> Input {
    let binding = fs::read_to_string(filename)
        .unwrap();
    let (ords, updates) = binding.split_once("\n\n")
        .unwrap();
    let updates = updates.split('\n').map(|line| {
        line.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
    }).collect();
    let mut maps: HashMap<i32, HashSet<i32>> = HashMap::new();
    for s in ords.split('\n') {
        let (f, l) = s.split_once('|').unwrap();
        let f = f.parse::<i32>().unwrap();
        let l = l.parse::<i32>().unwrap();
        match maps.get_mut(&f) {
            Some(b) => {b.insert(l);},
            None => {maps.insert(f, HashSet::from([l]));},
        };
    }
    (maps, updates)
}

fn task_one(mut input: Input) -> i32 {
    let (ords, updates) = input;
    let mut s = 0;
    'outer: for u in updates {
        for i in 0..u.len() {
            for j in (i+1)..u.len() {
                let Some(r) = ords.get(&u[j]) else {continue;};
                if !r.contains(&u[i]) {
                    continue;
                }
                continue 'outer;
            }
        }
        s += u[u.len()/2];
    }
    s
}

fn task_two(mut input: Input) -> i32 {
    let (ords, updates) = input;
    let mut s = 0;
    'outer: for mut u in updates {
        for i in 0..u.len() {
            for j in (i+1)..u.len() {
                let Some(r) = ords.get(&u[j]) else {continue;};
                if !r.contains(&u[i]) {
                    continue;
                }
                u.sort_by(|a, b| {
                    let Some(r) = ords.get(a) else {return Ordering::Equal;};
                    if r.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                s += u[u.len()/2];
                continue 'outer;
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
        assert_eq!(143, task_one(input.clone()));
        assert_eq!(123, task_two(input));
    }
}
