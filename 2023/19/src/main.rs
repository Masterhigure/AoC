#![allow(unused)]
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Range;

type Input = (HashMap<String, Map>, Vec<Part>);

struct Map {
    m: Vec<(usize,Box<dyn Fn(&Part) -> Option<String>>)>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let s = s.strip_suffix('}').unwrap();
        let mut m: Vec<(usize, Box<dyn Fn(&Part) -> Option<String>>)> = Vec::new();
        for c in s.split(',') {
            if let Some((cond, target)) = c.split_once(':') {
                let (index, value) = cond.split_once(|c| c == '>' || c == '<').unwrap();
                let index = match index {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => unreachable!(),
                };
                let value = value.parse::<i32>().unwrap();
                let target = target.to_string();
                if cond.contains('<') {
                    m.push((index,Box::new(move |&p| {
                        if p.0[index] < value {
                            Some(target.clone())
                        } else {
                            None
                        }
                    })));
                } else {
                    m.push((index, Box::new(move |&p| {
                        if p.0[index] > value {
                            Some(target.clone())
                        } else {
                            None
                        }
                    })));
                }
            }
            else {
                let c1 = c.to_string();
                m.push((4,Box::new(move |_| Some(c1.clone()))));
            }
        }
        Ok(Self {m})
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Part([i32; 4]);

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let s = s.strip_prefix('{').unwrap();
        let s = s.strip_suffix('}').unwrap();
        let mut i = s.split(',');
        Ok(Self([
            i.next().unwrap().split_once('=').unwrap().1.parse().unwrap(),
            i.next().unwrap().split_once('=').unwrap().1.parse().unwrap(),
            i.next().unwrap().split_once('=').unwrap().1.parse().unwrap(),
            i.next().unwrap().split_once('=').unwrap().1.parse().unwrap()
        ]))
    }
}

fn read_input(filename: &str) -> Input {
    let file = fs::read_to_string(filename).unwrap();
    let (maps, parts) = file.split_once("\n\n").unwrap();
    (
        maps.split('\n')
            .map(|s| s.split_once('{').unwrap())
            .map(|(s, t)| (s.to_string(), t.parse().unwrap())).collect(),
        parts.split('\n').map(|s| s.parse().unwrap()).collect()
    )
}

fn task_one(input: &Input) -> i32 {
    let mut s = 0;
    for &part in &input.1 {
        let mut cur = "in".to_string();
        while &cur != "R" && &cur != "A" {
            let mut new = String::new();
            for (_, m) in &input.0.get(&cur).unwrap().m {
                if let Some(n) = m(&part) {
                    new = n;
                    break;
                }
            }
            cur = new;
        }
        if &cur == "A" {
            s += part.0.iter().sum::<i32>();
        }
    }
    s
}

fn task_two(mut input: Input) -> usize {
    let mut unfinished = vec![("in".to_string(), [1..4001, 1..4001, 1..4001, 1..4001])];
    let mut a = Vec::new();
    while let Some((w, p)) = unfinished.pop() {
        if &w == "A" {
            a.push(p);
            continue;
        }
        if &w == "R" {
            continue;
        }
        let mut lower = Part([p[0].start, p[1].start, p[2].start, p[3].start]);
        let mut upper = Part([p[0].end, p[1].end, p[2].end, p[3].end]);
        for (i, m) in &input.0.get(&w).unwrap().m {
            let i = *i;
            if i == 4 {
                unfinished.push((m(&lower).unwrap(), [
                        lower.0[0]..upper.0[0],
                        lower.0[1]..upper.0[1],
                        lower.0[2]..upper.0[2],
                        lower.0[3]..upper.0[3],
                    ]));
                break;
            }
            if (0..4).any(|j| p[j].is_empty()) {
                break;
            }
            match (m(&lower), m(&upper)) {
                (None, None) => continue,
                (Some(s), Some(_)) => {
                    unfinished.push((s, [
                        lower.0[0]..upper.0[0],
                        lower.0[1]..upper.0[1],
                        lower.0[2]..upper.0[2],
                        lower.0[3]..upper.0[3],
                    ]));
                    break;
                },
                (Some(s), None) => {
                    let mut l = lower.0[i];
                    let start_l = l;
                    let mut u = upper.0[i] - 1;
                    let mut c = (l + u)/2;
                    loop {
                        if l + 1 >= u {
                            lower.0[i] = u;
                            break;
                        }
                        c = l + (u-l)/2;
                        lower.0[i] = c;
                        if m(&lower).is_some() {
                            l = c;
                        } else {
                            u = c;
                        }
                        lower.0[i] = start_l;
                    }
                    let mut p_new = [
                        lower.0[0]..upper.0[0],
                        lower.0[1]..upper.0[1],
                        lower.0[2]..upper.0[2],
                        lower.0[3]..upper.0[3],
                    ];
                    lower.0[i] = u;
                    p_new[i] = start_l..u;
                    unfinished.push((s, p_new));
                },
                (None, Some(s)) => {
                    let mut l = lower.0[i];
                    let mut u = upper.0[i] - 1;
                    let start_u = u;
                    let mut c = (l + u)/2;
                    loop {
                        if l + 1 >= u {
                            upper.0[i] = l + 1;
                            break;
                        }
                        c = (l + u)/2;
                        upper.0[i] = c;
                        if m(&upper).is_some() {
                            u = c;
                        } else {
                            l = c;
                        }
                        upper.0[i] = start_u + 1;
                    }
                    let mut p_new = [
                        lower.0[0]..upper.0[0],
                        lower.0[1]..upper.0[1],
                        lower.0[2]..upper.0[2],
                        lower.0[3]..upper.0[3],
                    ];
                    upper.0[i] = l + 1;
                    p_new[i] = (l+1)..(start_u + 1);
                    unfinished.push((s, p_new));
                },
            }
        }
    }
    let mut s = 0;
    for p in a {
        let mut prod = 1;
        for r in p {
            prod *= r.len();
        }
        s += prod;
    }
    s
}

fn main() {
    let input = read_input("input.txt");
    println!("Task one: {}", task_one(&input));
    println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(19114, task_one(&input));
        assert_eq!(167409079868000, task_two(input));
    }
}
