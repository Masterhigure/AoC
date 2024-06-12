#![allow(unused)]
use std::fs;

type Input = Vec<Vec<[usize; 2]>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '|' => [0, 2],
                    '-' => [1, 3],
                    'J' => [0, 3],
                    'F' => [1, 2],
                    'L' => [0, 1],
                    '7' => [2, 3],
                    '.' => [4, 4],
                    'S' => [5, 5],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn task_one(mut input: Input) -> i32 {
    let mut start = [0, 0];
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let mut line = vec![-1_i32; input[i].len()];
        if let Some(j) = input[i].iter().position(|&x| x == [5, 5]) {
            start = [i, j];
            line[j] = 0;
        }
        map.push(line);
    }
    let mut curr = start;
    if start[0] > 0 && input[start[0] - 1][start[1]].contains(&2) {
        curr[0] -= 1;
    } else if start[0] + 1 < input.len() && input[start[0] + 1][start[1]].contains(&0) {
        curr[0] += 1;
    } else if start[1] > 0 && input[start[0]][start[1] - 1].contains(&1) {
        curr[1] -= 1;
    } else if start[1] + 1 < input[0].len() && input[start[0]][start[1] + 1].contains(&3) {
        curr[1] += 1;
    } else {
        unreachable!();
    }
    let mut dist = 0;
    while curr != start {
        dist += 1;
        map[curr[0]][curr[1]] = dist;
        if curr[0] > 0 
            && input[curr[0]][curr[1]].contains(&0) 
            && map[curr[0] - 1][curr[1]] == -1 
        {
            curr[0] -= 1;
        } else if curr[0] + 1 < input.len()
            && input[curr[0]][curr[1]].contains(&2)
            && map[curr[0] + 1][curr[1]] == -1
        {
            curr[0] += 1;
        } else if curr[1] > 0
            && input[curr[0]][curr[1]].contains(&3)
            && map[curr[0]][curr[1] - 1] == -1
        {
            curr[1] -= 1;
        } else if curr[1] + 1 < input[0].len()
            && input[curr[0]][curr[1]].contains(&1)
            && map[curr[0]][curr[1] + 1] == -1
        {
            curr[1] += 1
        }
        else {
            break;
        }
    }
    (dist + 1) / 2
}

fn task_two(mut input: Input) -> isize {
    let mut start = [0, 0];
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let mut line = vec![-1_i32; input[i].len()];
        if let Some(j) = input[i].iter().position(|&x| x == [5, 5]) {
            start = [i, j];
            line[j] = 0;
        }
        map.push(line);
    }
    let mut vert_corr: isize = 0;
    let mut j_corr: isize = 0;
    let mut seven_corr: isize = 0;

    let mut area: isize = 0;
    let mut curr = start;
    if start[0] > 0 && input[start[0] - 1][start[1]].contains(&2) {
        curr[0] -= 1;
        area += <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
    } else if start[0] + 1 < input.len() && input[start[0] + 1][start[1]].contains(&0) {
        curr[0] += 1;
        vert_corr += 1;
        area -= <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
    } else if start[1] > 0 && input[start[0]][start[1] - 1].contains(&1) {
        curr[1] -= 1;
    } else if start[1] + 1 < input[0].len() && input[start[0]][start[1] + 1].contains(&3) {
        curr[1] += 1;
    } else {
        unreachable!();
    }
    
    // Kategoriser start-feltet
    let mut s_cat = Vec::new();

    if start[0] > 0 && input[start[0] - 1][start[1]].contains(&2) {
        s_cat.push(0);
    } if start[0] + 1 < input.len() && input[start[0] + 1][start[1]].contains(&0) {
        s_cat.push(2);
    } if start[1] > 0 && input[start[0]][start[1] - 1].contains(&1) {
        s_cat.push(3);
    } if start[1] + 1 < input[0].len() && input[start[0]][start[1] + 1].contains(&3) {
        s_cat.push(1);
    }
    
    let mut diag_corr_flag = false;
    if s_cat == vec![0, 3] {
        j_corr += <usize as TryInto<isize>>::try_into(start[1]).unwrap();
        diag_corr_flag = true;
    } else if s_cat == vec![0, 1] {
        seven_corr -= <usize as TryInto<isize>>::try_into(start[1]).unwrap();
        diag_corr_flag = true;
    } else if s_cat == vec![2, 1] {
        j_corr -= <usize as TryInto<isize>>::try_into(start[1]).unwrap();
        diag_corr_flag = true;
    } else if s_cat == vec![2, 3] {
        seven_corr += <usize as TryInto<isize>>::try_into(start[1]).unwrap();
        diag_corr_flag = true;
    }

    if diag_corr_flag {
        vert_corr = 0;
    }

    let mut dist = 0;
    while curr != start {
        dist += 1;
        map[curr[0]][curr[1]] = dist;
        let mut diag_corr_flag = false;
        if input[curr[0]][curr[1]] == [0, 3] {
            j_corr += <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
        } else if input[curr[0]][curr[1]] == [2, 3] {
            seven_corr += <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
        } else if input[curr[0]][curr[1]] == [1, 2] {
            j_corr -= <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
        } else if input[curr[0]][curr[1]] == [0, 1] {
            seven_corr -= <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
        }

        if curr[0] > 0 
            && input[curr[0]][curr[1]].contains(&0) 
            && map[curr[0] - 1][curr[1]] == -1 
        {
            area += <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
            curr[0] -= 1;
        } else if curr[0] + 1 < input.len()
            && input[curr[0]][curr[1]].contains(&2)
            && map[curr[0] + 1][curr[1]] == -1
        {
            curr[0] += 1;
            vert_corr += 1;
            area -= <usize as TryInto<isize>>::try_into(curr[1]).unwrap();
        } else if curr[1] > 0
            && input[curr[0]][curr[1]].contains(&3)
            && map[curr[0]][curr[1] - 1] == -1
        {
            curr[1] -= 1;
        } else if curr[1] + 1 < input[0].len()
            && input[curr[0]][curr[1]].contains(&1)
            && map[curr[0]][curr[1] + 1] == -1
        {
            curr[1] += 1
        }
        else {
            dbg!(&curr, &start, input[curr[0]][curr[1]], &s_cat, area, vert_corr, seven_corr);
            curr = start;
        }
    }
    if area < 0 {
        -area - vert_corr - seven_corr
    } else {
        area - vert_corr - j_corr
    }
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

        let input3 = read_input("example_input_3.txt");
        let input4 = read_input("example_input_4.txt");
        assert_eq!(4, task_one(input.clone()));
        assert_eq!(8, task_one(input2.clone()));
        assert_eq!(1, task_two(input));
        assert_eq!(1, task_two(input2));
        assert_eq!(4, task_two(input3));
        assert_eq!(8, task_two(input4));
    }
}
