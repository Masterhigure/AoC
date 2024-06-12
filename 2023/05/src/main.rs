#![allow(unused)]
#![feature(slice_as_chunks, extract_if)]
use std::fs;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
struct Chart {
    dest: Range<i64>,
    source: Range<i64>,
}

impl FromStr for Chart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns: Vec<i64> = s.split(' ').map(str::parse).map(Result::unwrap).collect();
        Ok(Self {
            dest: ns[0]..(ns[0] + ns[2]),
            source: ns[1]..(ns[1] + ns[2]),
        })
    }
}

type Map = Vec<Chart>;

type Input = (Vec<i64>, Vec<Map>);

fn read_input(filename: &str) -> Input {
    let f = fs::read_to_string(filename)
        .unwrap();
    let (s, m) = f.split_once("\n\n")
        .unwrap();
    let s = s
        .split_once(": ").unwrap().1
        .split(' ')
        .map(str::parse::<i64>).map(Result::unwrap)
        .collect();
    let m = m.split("\n\n")
        .map(|s| {
            s.split_once('\n').unwrap().1
                .split("\n")
                .map(str::parse).map(Result::unwrap)
                .collect()
        }).collect();
    (s, m)
}

fn source_to_dest_one(source: &i64, m: &Map) -> i64 {
    for c in m {
        if c.source.contains(source) {
            return c.dest.start + source - c.source.start;
        }
    }
    return *source;
}

fn task_one(mut input: Input) -> i64 {
    let mut min_loc = i64::MAX;
    for seed in input.0 {
        let mut val = seed;
        for map in &input.1 {
            val = source_to_dest_one(&val, &map);
        }
        min_loc = min_loc.min(val); 
    }
    min_loc
}

fn intersects(r1: &Range<i64>, r2: &Range<i64>) -> bool {
    r1.start < r2.end && r2.start < r1.end
}

fn source_to_dest_two(mut source: Range<i64>, map: &Map) -> Vec<Range<i64>> {
    let mut sources = vec![source];
    let mut dests = Vec::new();
    for chart in map {
        let touched: Vec<_> = sources.extract_if(|s| intersects(s, &chart.source)).collect();
        for t in touched {
            if t.start < chart.source.start {
                sources.push(t.start..t.end.min(chart.source.start));
            }
            if t.end > chart.source.end {
                sources.push(chart.source.end.max(t.start)..t.end);
            }
            let t_c_overlap = t.start.max(chart.source.start)..t.end.min(chart.source.start);
            let dest_start = chart.dest.start + 0.max(t.start - chart.source.start);
            let dest_end = chart.dest.end - 0.max(chart.source.end - t.end);
            dests.push(dest_start..dest_end);
        }
    }
    dests.append(&mut sources);
    dests
}

fn task_two(mut input: Input) -> i64 {
    let seeds: Vec<Range<i64>> = input.0.as_chunks::<2>().0.iter()
        .map(|c| c[0]..(c[0] + c[1] + 1)).collect();
    let mut locs: Vec<Range<i64>> = Vec::new();
    for seed in seeds {
        let mut sources = vec![seed];
        for m in &input.1 {
            let mut dests = Vec::new();
            for s in sources {
                dests.append(&mut source_to_dest_two(s, &m));
            }
            sources = dests;
        }
        locs.append(&mut sources);
    }
    locs.iter().map(|r| r.start).min().unwrap()
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
        assert_eq!(35, task_one(input.clone()));
        assert_eq!(46, task_two(input));
    }
}
