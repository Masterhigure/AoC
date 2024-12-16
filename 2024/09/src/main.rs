#![allow(unused)]
use std::fs;

type Input = Vec<usize>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as usize)
        .collect()
}

fn task_one(mut input: Input) -> usize {
    let mut summation = 0;
    let mut i_front = 0;
    let mut i_back = input.len()-1;
    let mut m_front = 0;
    let mut m_back = input.iter().sum::<usize>()- 1;
    while m_front < m_back && i_front <= i_back {
        if input[i_front] == 0 {
            i_front += 1;
            continue;
        }
        if i_front % 2 == 0 {
            summation += m_front*(i_front/2);
            m_front += 1;
            input[i_front] -= 1;
        } else {
            if input[i_back] == 0 {
                i_back -= 2;
                continue;
            }
            summation += m_front*(i_back / 2);
            m_back -= 1;
            m_front += 1;
            input[i_front] -= 1;
            input[i_back] -= 1;
        }
    }
    summation
}

fn task_two(mut input: Input) -> usize {
    let mut summation = 0;
    let mut i_back = input.len() - 1;
    let mut fills = vec![0; input.len()];
    'outer: while i_back > 1 {
        let s = input[i_back];
        for i in 0..=(i_back/2 - 1) {
            let i = 2*i + 1;
            if input[i] >= s {
                let pos = input[0..i].iter().sum::<usize>() + fills[0..=i].iter().sum::<usize>();
                for k in pos..(pos + input[i_back]) {
                    summation += k*(i_back / 2);
                    input[i] -= 1;
                    fills[i] += 1;
                    input[i_back] -= 1;
                }
                i_back -= 2;
                continue 'outer;
            }
        }
        let pos = input[0..i_back].iter().sum::<usize>() + fills[0..=i_back].iter().sum::<usize>();
        for k in pos..(pos+input[i_back]) {
            summation += k*(i_back / 2);
        }
        i_back -= 2;
    }
    summation
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
        assert_eq!(1928, task_one(input.clone()));
        assert_eq!(2858, task_two(input));
    }
}
