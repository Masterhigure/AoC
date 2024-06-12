#![allow(unused)]
use std::fs;

type Input = Vec<String>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn task_one(mut input: Input) -> i32 {
    let mut s = 0;
    for i in input {
        let mut h = 0;
        for b in i.as_bytes() {
            h += *b as i32;
            h *= 17;
            h %= 256;
        }
        s += h;
    }
    s
}


fn task_two(mut input: Input) -> usize{
    let mut boxes = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::<(String,usize)>::new());
    }
    for mut i in input {
        let cmd_pos = i.chars().position(|c| c == '=' || c == '-').unwrap();
        let cmd = i.split_off(cmd_pos);
        let mut h = 0;
        for b in i.as_bytes() {
            h += *b as usize;
            h *= 17;
            h %= 256;
        }
        match &cmd[..1] {
            "=" => {
                let focus = cmd[1..].parse::<usize>().unwrap();
                if let Some(l) = boxes[h].iter().position(|(s, _)| *s == i) {
                    boxes[h][l] = (i, focus);
                } else {
                    boxes[h].push((i, focus));
                }
            },
            _ => {
                if let Some(l) = boxes[h].iter().position(|(s, _)| *s == i) {
                    boxes[h].remove(l);
                }
            },
        }
    }
    let mut s = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (j, (_, focus)) in b.into_iter().enumerate() {
            s += (i+1)*(j+1)*focus;
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
        assert_eq!(1320, task_one(input.clone()));
        assert_eq!(145, task_two(input));
    }
}
