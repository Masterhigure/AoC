#![allow(unused)]
use std::fs;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: u32,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { value: s
            .chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    c.to_digit(10).unwrap()
                } else {
                    match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 1,
                        'T' => 10,
                        _ => unreachable!(),
                    }
                }
            }).next().unwrap() 
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: i32,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (h, b) = s.split_once(' ').unwrap();
        let mut cards = <[Card; 5]>::default();
        for (i, c) in h.split_terminator("").skip(1).enumerate() {
            cards[i] = c.parse().unwrap();
        }
        Ok(Self { cards, bid: b.parse().unwrap() })
    }
}

impl Hand {
    fn value(&self) -> i32 {
        let mut counts = vec![0_i32; 13];
        for c in self.cards {
            counts[c.value as usize -2] += 1
        }
        counts.sort_by(|u, v| v.cmp(u) );
        if counts[0] == 5 {
            6
        } else if counts[0] == 4 {
            5
        } else if counts[0] == 3 && counts[1] == 2 {
            4
        } else if counts[0] == 3 {
            3
        } else if counts[0] == 2 && counts[1] == 2 {
            2
        } else if counts[0] == 2 {
            1
        } else {
            0
        }
    }

    fn value_joker(&self) -> i32 {
        let mut counts = vec![0_i32; 15];
        for c in self.cards {
            counts[c.value as usize] += 1;
        }
        let j = counts.swap_remove(1);
        counts.sort_by(|u, v| v.cmp(u));
        counts[0] += j;
        if counts[0] == 5 {
            6
        } else if counts[0] == 4 {
            5
        } else if counts[0] == 3 && counts[1] == 2 {
            4
        } else if counts[0] == 3 {
            3
        } else if counts[0] == 2 && counts[1] == 2 {
            2
        } else if counts[0] == 2 {
            1
        } else {
            0
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value_joker() != other.value_joker() {
            self.value_joker().cmp(&other.value_joker())
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


type Input = Vec<Hand>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    input.sort_unstable();
    input.iter().enumerate().map(|(i, h)| (i+1)*h.bid as usize).sum::<usize>()
}

fn main() {
    let input = read_input("input.txt");
    println!("Task: {}", task_one(input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(5904, task_one(input));
        //assert_eq!(0, task_two(input));
    }
}
