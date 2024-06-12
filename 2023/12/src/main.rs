#![allow(unused)]
use std::fs;
use std::str::FromStr;
use std::ops::Range;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
struct Record {
    r1: Vec<char>,
    r2: Vec<usize>,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once(' ').unwrap();
        Ok(Record {
            r1: s1.chars().collect(),
            r2: s2.split(',').map(str::parse).map(Result::unwrap).collect()
        })
    }
}

type Input = Vec<Record>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn combinations(r: Range<usize>, n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut k = r.start;
    let mut c = vec![];
    while k + n <= r.end {
        for mut v in combinations((k+1)..r.end, n-1) {
            v.push(k);
            c.push(v);
        }
        k += 1;
    }
    c
}

fn fits_data(r1: &[char], r2: &[usize]) -> bool {
    r1.split(|&c| c == '.')
        .filter(|chunk| chunk.len() > 0)
        .map(|chunk| chunk.len())
        .collect::<Vec<_>>() == *r2
}

fn task_one(mut input: Input) -> usize {
    let mut s = 0;
    for Record{ref mut r1, r2} in &mut input[..] {
        let mut n = 0;
        let qs = r1.iter().enumerate()
            .filter(|(_, &c)| c == '?').map(|(n, _)| n)
            .collect::<Vec<_>>();
        let num_h = r1.iter().filter(|&&c| c == '#').count();
        for c in combinations(0..qs.len(), r2.iter().sum::<usize>() - num_h) {
            for (j, &q) in qs.iter().enumerate() {
                if c.contains(&j) {
                    r1[q] = '#';
                } else {
                    r1[q] = '.';
                }
            }
            if fits_data(&r1, &r2) {
                n += 1;
            }
        }
        s += n;
    }
    s
}

fn num_solutions(r1: &[char], r2: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if r2.len() == 0 {
        if r1.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }
    if cache.contains_key(&(r1.len(), r2.len())) {
        *cache.get(&(r1.len(), r2.len())).unwrap()
    } else {
        let mut sum = 0;
        let s = r2.iter().map(|&n| n+1).sum::<usize>();
        let r = r2[0];
        for i in 0..(r1.len() - s + 1) {
            if r1[i..(i+r)].iter().all(|&c| c != '.') && r1[i + r] != '#' {
                sum += num_solutions(&r1[(i+r+1)..], &r2[1..], cache);
            }
            if r1[i] == '#' {
                break;
            }
        }
        cache.insert((r1.len(), r2.len()), sum);
        sum
    }
}

fn num_solutions_memo(r1: &[char], r2: &[usize]) -> usize {
    let mut springdex = 0;
    let mut placements = vec![0; r2.len()];
    let mut s = 0;
    let cumsum = (0..r2.len())
        .map(|i| r2[i..].iter().map(|&n| n+1).sum::<usize>())
        .collect::<Vec<_>>();
    'outer: loop {
        let p_s = placements[springdex];
        let p_e = placements[springdex] + r2[springdex];
        if r1[p_s..p_e].iter().all(|&c| c == '#' || c == '?')
            && r1[p_e] != '#' {
            if springdex + 1 == r2.len() {
                if !r1[(p_e + 1)..].contains(&'#') {
                    s += 1;
                }
                if r1[p_s] == '#' {
                    loop {
                        if springdex == 0 {
                            break 'outer;
                        } else {
                            springdex -= 1;
                            if r1[placements[springdex]] != '#' {
                                placements[springdex] += 1;
                                break;
                            }
                        }
                    }
                } else {
                    placements[springdex] += 1;
                }
            } else {
                springdex += 1;
                placements[springdex] = p_e + 1;
            }
        } else if r1[p_s] == '#' {
            loop {
                if springdex == 0 {
                    break 'outer;
                } else {
                    springdex -= 1;
                    if r1[placements[springdex]] != '#' {
                        placements[springdex] += 1;
                        break;
                    }
                }
            }
        } else {
            placements[springdex] += 1;
        }
        while placements[springdex] + cumsum[springdex] > r1.len() {
            if springdex == 0 {
                break 'outer;
            } else {
                springdex -= 1;
                while r1[placements[springdex]] == '#' {
                    if springdex == 0 {
                        break 'outer;
                    }
                    springdex -= 1;
                }
                placements[springdex] += 1;
            }
        }
    }
    s
}

fn task_two(mut input: Input) -> usize {
    let mut s = 0;
    for Record{ref mut r1, r2} in &mut input[..] {
        let mut cache = HashMap::new();
        r1.push('?');
        *r1 = r1.repeat(5);
        r1.pop();
        r1.push('.');

        let r2 = r2.repeat(5);
        let n = num_solutions(&r1, &r2, &mut cache);
        s += n;
    }
    s
}

fn main() {
    let input = read_input("input.txt");
    //println!("Task one: {}", task_one(input.clone()));
    println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(21, task_one(input.clone()));
        assert_eq!(525152, task_two(input));
    }
}
