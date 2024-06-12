#![allow(unused)]
use std::fs;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube{ x: usize, y: usize, z: usize }

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(',');
        Ok(Self{
            x: i.next().unwrap().parse().unwrap(),
            y: i.next().unwrap().parse().unwrap(),
            z: i.next().unwrap().parse().unwrap(),
        })
    }
}

impl Cube {
    fn range(&self, other: &Self) -> Vec<Self> {
        let mut r = Vec::new();
        if self.x == other.x && self.y == other.y {
            for z in self.z.min(other.z)..=self.z.max(other.z) {
                r.push(Self{ z, ..*self });
            }
        } else if self.x == other.x && self.z == other.z {
            for y in self.y.min(other.y)..=self.y.max(other.y) {
                r.push(Self{ y, ..*self });
            }
        } else if self.z == other.z && self.y == other.y {
            for x in self.x.min(other.x)..=self.x.max(other.x) {
                r.push(Self{ x, ..*self });
            }
        }
        r
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Brick {
    cubes: Vec<Cube>,
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('~').unwrap();
        Ok(Brick{ cubes: Cube::range(&l.parse().unwrap(), &r.parse().unwrap()) })
    }
}

impl Brick {
    fn intersects(&self, other: &Self) -> bool {
        self.cubes.iter().any(|c| other.cubes.contains(c))
    }

    fn move_down(&mut self) -> bool {
        if self.cubes.iter().any(|&c| c.z <= 1) {
            return false;
        }
        for ref mut c in &mut self.cubes {
            c.z -= 1;
        }
        true
    }

    fn move_up(&mut self) {
        for ref mut c in &mut self.cubes {
            c.z += 1;
        }
    }
}

type Input = Vec<Brick>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap() )
    .collect()
}

fn task_one(mut input: Input) -> i32 {
    input.sort_unstable_by_key(|b| b.cubes.iter().map(|c| c.z).min().unwrap());
    let mut unfinished = true;
    while unfinished {
        unfinished = false;
        for i in 0..input.len() {
            for j in 0.. {
                let bottom = input[i].move_down();
                let isct = input[..i].iter().any(|b| b.intersects(&input[i]));
                if !bottom || isct {
                    if j > 0 {
                        unfinished = true;
                    }
                    if isct {
                        input[i].move_up();
                    }
                    break;
                }
            }
        }
        input.sort_unstable_by_key(|b| b.cubes.iter().map(|c| c.z).min().unwrap());
    }
    let mut lower_to_upper: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut upper_to_lower: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..input.len() {
        input[i].move_up();
        for j in (i+1)..input.len() {
            if input[j].intersects(&input[i]) {
                if let Some(c) = lower_to_upper.get_mut(&i) {
                    c.push(j);
                } else {
                    lower_to_upper.insert(i, vec![j]);
                }
                if let Some(c) = upper_to_lower.get_mut(&j) {
                    c.push(i);
                } else {
                    upper_to_lower.insert(j, vec![i]);
                }
            }
        }
    }
    for i in 0..input.len() {
        if !lower_to_upper.contains_key(&i) {
            lower_to_upper.insert(i, vec![]);
        }
        if !upper_to_lower.contains_key(&i) {
            upper_to_lower.insert(i, vec![]);
        }
    }
    let mut s = 0;
    for (i, v) in lower_to_upper {
        if v.iter().all(|j| upper_to_lower.get(&j).unwrap().iter().any(|&k| k != i)) {
            s += 1;
        }
    }
    s
}

fn task_two(mut input: Input) -> usize {
    input.sort_unstable_by_key(|b| b.cubes.iter().map(|c| c.z).min().unwrap());
    let mut unfinished = true;
    while unfinished {
        unfinished = false;
        for i in 0..input.len() {
            for j in 0.. {
                let bottom = input[i].move_down();
                let isct = input[..i].iter().any(|b| b.intersects(&input[i]));
                if !bottom || isct {
                    if j > 0 {
                        unfinished = true;
                    }
                    if isct {
                        input[i].move_up();
                    }
                    break;
                }
            }
        }
        input.sort_unstable_by_key(|b| b.cubes.iter().map(|c| c.z).min().unwrap());
    }
    let mut lower_to_upper: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut upper_to_lower: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..input.len() {
        input[i].move_up();
        for j in (i+1)..input.len() {
            if input[j].intersects(&input[i]) {
                if let Some(c) = lower_to_upper.get_mut(&i) {
                    c.push(j);
                } else {
                    lower_to_upper.insert(i, vec![j]);
                }
                if let Some(c) = upper_to_lower.get_mut(&j) {
                    c.push(i);
                } else {
                    upper_to_lower.insert(j, vec![i]);
                }
            }
        }
        input[i].move_down();
    }
    for i in 0..input.len() {
        if !lower_to_upper.contains_key(&i) {
            lower_to_upper.insert(i, vec![]);
        }
        if !upper_to_lower.contains_key(&i) {
            upper_to_lower.insert(i, vec![]);
        }
    }
    let mut s = 0;
    for i in 0..input.len() {
        let mut fallen = HashSet::new();
        fallen.insert(i);
        let mut prev_len = 0;
        while fallen.len() != prev_len {
            prev_len = fallen.len();
            for j in 0..input.len() {
                if fallen.contains(&j) {
                    continue;
                }
                let supps = upper_to_lower.get(&j).unwrap();
                if supps.iter().all(|k| fallen.contains(k)) 
                    && input[j].cubes.iter().all(|c| c.z > 1) {
                        fallen.insert(j);
                }
            }
        }
        s += fallen.len() - 1;
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
        assert_eq!(5, task_one(input.clone()));
        assert_eq!(7, task_two(input));
    }
}
