#![allow(unused)]
use std::fs;
use std::collections::HashSet;
use std::ops::{Index, IndexMut, Deref, DerefMut};

type Input = Vec<Vec<char>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect()
}

fn task_one(mut input: Input, steps: usize) -> usize {
    let mut poss: HashSet<(usize, usize)> = HashSet::new();
    let start = input.iter().enumerate()
        .filter_map(|(i, v)| v.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .next().unwrap();
    poss.insert(start);
    input[start.0][start.1] = '.';
    for _ in 0..steps {
        let mut nexts = HashSet::new();
        for &(x, y) in &poss {
            if x > 0 && input[x-1][y] == '.' {
                nexts.insert((x-1, y));
            }
            if y > 0 && input[x][y-1] == '.' {
                nexts.insert((x, y-1));
            }
            if x + 1 < input.len() && input[x+1][y] == '.' {
                nexts.insert((x+1, y));
            }
            if y + 1 < input[0].len() && input[x][y+1] == '.' {
                nexts.insert((x, y + 1));
            }
        }
        poss = nexts;
    }
    poss.len()
}

struct GardenCoverage {
    v: Vec<usize>,
}

impl Index<usize> for GardenCoverage {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.v.len() {
            &self.v[index]
        } else {
            &self.v[self.v.len() - 1 - (index - self.v.len() + 1)%2]
        }
    }
}

impl IndexMut<usize> for GardenCoverage {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        let l = self.v.len();
        if index < l {
            &mut self.v[index]
        } else {
            &mut self.v[l - 1 - (index - l + 1)%2]
        }
    }
}

impl Deref for GardenCoverage {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl DerefMut for GardenCoverage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

impl From<Vec<usize>> for GardenCoverage {
    fn from(v: Vec<usize>) -> Self {
        Self {v}
    }
}

fn task_two(mut input: Input) -> usize {
    let total_steps = 26501365;
    let s = input.iter().enumerate()
            .filter_map(|(i, v)| v.iter().position(|&c| c == 'S').map(|j| (i, j)))
            .next().unwrap();
    println!("{s:?}");
    let starts = vec![
        s,
        (0,0),
        (input.len() - 1, 0),
        (0, input[0].len() - 1),
        (input.len() - 1, input[0].len() - 1),
        (s.0, 0),
        (s.0, input[0].len() - 1),
        (0, s.1),
        (input.len() - 1, s.1),
    ];
    let mut filleds = vec![];
    for &start in &starts {
        let mut poss: HashSet<(usize, usize)> = HashSet::new();
        poss.insert(start);
        input[start.0][start.1] = '.';
        let mut filled: GardenCoverage = vec![1].into();
        for i in 0.. {
            let mut nexts = HashSet::new();
            for &(x, y) in &poss {
                if x > 0 && input[x-1][y] == '.' {
                    nexts.insert((x-1, y));
                }
                if y > 0 && input[x][y-1] == '.' {
                    nexts.insert((x, y-1));
                }
                if x + 1 < input.len() && input[x+1][y] == '.' {
                    nexts.insert((x+1, y));
                }
                if y + 1 < input[0].len() && input[x][y+1] == '.' {
                    nexts.insert((x, y + 1));
                }
            }
            poss = nexts;
            filled.push(poss.len());
            if i >= 2 && poss.len() == filled[i-1] {
                filleds.push(filled);
                break;
            }
        }
    }
    let mut covered = filleds[0][total_steps];
    for i in 1..9 {
        let mut steps_taken = if i < 5 { 132 } else { 66 };
        let mut layer = 1;
        loop {
            let multiplier = if i < 5 { layer } else {1};
            covered += filleds[i][total_steps - steps_taken]*multiplier;
            layer += 1;
            steps_taken += 131;
            if steps_taken >= total_steps {
                break;
            }
        }
    }
    covered
}

fn main() {
    let input = read_input("input.txt");
    println!("Task one: {}", task_one(input.clone(), 64));
    println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(16, task_one(input.clone(), 6));
        //assert_eq!(0, task_two(input));
    }
}
