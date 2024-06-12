#![allow(unused)]
use std::fs;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use core::ops::Deref;

#[derive(Eq, PartialEq, Debug, Default, Clone)]
struct Relay {
    n: String,
    c: Class,
}

impl Hash for Relay {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.n.hash(state);
    }
}

impl Deref for Class {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        use Class::*;
        match self {
            Broad{ out: o } => o,
            Flip { out: o, .. } => o,
            Conj { out: o, .. } => o,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Class {
    Broad{ out: Vec<String> },
    Flip{ state: bool, out: Vec<String> },
    Conj{ state: HashMap<String, bool>, out: Vec<String>},
}

impl Default for Class {
    fn default() -> Self {
        Self::Broad{ out: Vec::new() }
   }
}

impl Relay {
    fn recv(&mut self, source: &str, pulse: bool) -> Vec<(String, String, bool)> {
        use Class::*;
        match &mut self.c {
            Broad { out: o } => o.iter().cloned()
                .map(|r| (self.n.to_string(), r, false))
                .collect(),
            Flip { state: ref mut s, out: o } => {
                if !pulse {
                    *s = !*s;
                    o.iter()
                        .map(|r| (self.n.to_string(), r.clone(), *s)).collect()
                } else {
                    Vec::new()
                }
            },
            Conj { state: ref mut m, out: o } => {
                m.insert(source.to_string(), pulse);
                let p = m.values().all(|&v| v);
                o.iter().cloned().map(|r| (self.n.to_string(), r, !p)).collect()
            },
        }
    }
}

type Input = HashMap<String, Relay>;

fn read_input(filename: &str) -> Input {
    use Class::*;
    let binding = fs::read_to_string(filename)
        .unwrap();
    let raw = binding.split('\n')
        .map(|s| s.split_once(" -> ").unwrap())
        .collect::<Vec<_>>();
    let mut output = HashMap::new();
    let mut conjs = Vec::new();
    for &(n, o) in &raw {
        let out = o.split(", ").map(str::to_string).collect::<Vec<_>>();
        if n == "broadcaster" {
            output.insert(n.to_string(), Relay{
                n: n.to_string(),
                c: Broad { out },
            });
        } else if &n[..1] == "%" {
            output.insert(n[1..].to_string(), Relay {
                n: n[1..].to_string(),
                c: Flip { state: false, out },
            });
        } else {
            assert_eq!(&n[..1], "&");
            conjs.push(n[1..].to_string());
            output.insert(n[1..].to_string(), Relay {
                n: n[1..].to_string(),
                c: Conj { state: HashMap::new(), out },
            });
        }
    }
    for s in conjs {
        let sources = output.values()
            .filter(|r| r.c.contains(&s))
            .map(|r| r.n.clone())
            .collect::<Vec<_>>();
        let cur = output.get_mut(&s).unwrap();
        match cur.c {
            Conj { ref mut state, .. } => {
                for k in sources {
                    state.insert(k, false);
                }
            },
            _ => unreachable!(),
        }
    }
    output
}

fn task_one(mut input: Input) -> usize {
    let mut lows = 0;
    let mut highs = 0;
    for j in 0..1000 {
        let mut pulses = vec![("button".to_string(), "broadcaster".to_string(), false)];
        let mut i = 0;
        while i < pulses.len() {
            let (source, target, pulse) = pulses[i].clone();
            i += 1;
            if !input.contains_key(&target) {
                continue;
            }
            pulses.append(&mut input.get_mut(&target).unwrap().recv(&source, pulse));
        }
        highs += pulses.iter().filter(|(_, _, p)| *p).count();
        lows += pulses.iter().filter(|(_, _, p)| !*p).count();
    }
    highs*lows
}

fn task_two(mut input: Input) -> usize {
    let mut jg = Vec::new();
    let mut hf = Vec::new();
    let mut jm = Vec::new();
    let mut rh = Vec::new();
    for j in 1..20_000 {
        if j%1_000_000 == 0 {
            println!("{}", j/1_000_000);
        }
        let mut pulses = vec![("button".to_string(), "broadcaster".to_string(), false)];
        let mut i = 0;
        while i < pulses.len() {
            let (source, target, pulse) = pulses[i].clone();
            i += 1;
            if !input.contains_key(&target) {
                continue;
            }
            if &source == "rh" && pulse {
                rh.push(j);
            }
            if &source == "jg" && pulse {
                jg.push(j);
            }
            if &source == "jm" && pulse {
                jm.push(j);
            }
            if &source == "hf" && pulse {
                hf.push(j);
            }
            pulses.append(&mut input.get_mut(&target).unwrap().recv(&source, pulse));
        }
    }
    println!("{:?}\n{:?}\n{:?}\n{:?}\n", jg, hf, jm, rh);
    let jgd = jg[1] - jg[0];
    let hfd = hf[1] - hf[0];
    let jmd = jm[1] - jm[0];
    let rhd = rh[1] - rh[0];

    println!("{:?}\n{:?}\n{:?}\n{:?}\n", jgd, hfd, jmd, rhd);

    for i in (jg.len() - 1)..10_000 {
        jg.push(jg[i] + jgd);
    }

    let mut two = jg.into_iter().filter(|&k| k%hfd == hf[0]%hfd).collect::<Vec<_>>();
    let twod = two[1] - two[0];
    for i in (two.len() - 1)..10_000 {
        two.push(two[i] + twod);
    }

    let mut three = two.into_iter().filter(|&k| k%jmd == jm[0]%jmd).collect::<Vec<_>>();
    let threed = three[1] - three[0];
    for i in (three.len() - 1)..10_000 {
        three.push(three[i] + threed);
    }
    
    let mut four = three.into_iter().filter(|&k| k%rhd == rh[0]%rhd).collect::<Vec<_>>();
    let fourd = four[1] - four[0];
    for i in (four.len() - 1)..10_000 {
        four.push(four[i] + fourd);
    }
    
    four[0]
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
        let input2 = read_input("example_input_2.txt");
        assert_eq!(32000000, task_one(input.clone()));
        assert_eq!(11687500, task_one(input2.clone()));
        //assert_eq!(0, task_two(input));
    }
}
