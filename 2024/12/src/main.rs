#![allow(unused)]
use std::fs;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect()
}

fn task_one(mut input: Input) -> usize {
    let mut fence = Vec::new();
    for i in 0..input.len() {
        let mut fence_row = Vec::new();
        for j in 0..input[i].len() {
            let mut fences = 0;
            let c = input[i][j];
            if i == 0 || input[i-1][j] != c {
                fences += 1;
            }
            if i + 1 == input.len() || input[i+1][j] != c {
                fences += 1;
            }
            if j == 0 || input[i][j-1] != c {
                fences += 1;
            }
            if j + 1 == input.len() || input[i][j+1] != c {
                fences += 1;
            }
            fence_row.push(fences);
        }
        fence.push(fence_row);
    }
    let mut unchecked = Vec::from([(0,0)]);
    let mut s = 0;
    while let Some((i, j)) = unchecked.pop() {
        let c = input[i][j];
        if c == '.' {
            continue;
        }
        let mut region = vec![(i,j)];
        let mut k = 0;
        while k < region.len() {
            let (x, y) = region[k];
            if x > 0 {
                if input[x-1][y] == c  {
                    if !region.contains(&(x-1, y)) {
                        region.push((x-1, y));
                    }
                } else {
                    if input[x-1][y] != '.' {
                        unchecked.push((x-1, y));
                    }
                }
            }
            if x + 1 < input.len() {
                if input[x+1][y] == c  {
                    if !region.contains(&(x+1, y)) {
                        region.push((x+1, y));
                    }
                } else {
                    if input[x+1][y] != '.' {
                        unchecked.push((x+1, y));
                    }
                }
            }
            if y > 0 {
                if input[x][y-1] == c  {
                    if !region.contains(&(x, y-1)) {
                        region.push((x, y-1));
                    }
                } else {
                    if input[x][y-1] != '.' {
                        unchecked.push((x, y-1));
                    }
                }
            }
            if y + 1 < input[0].len() {
                if input[x][y+1] == c  {
                    if !region.contains(&(x, y+1)) {
                        region.push((x, y+1));
                    }
                } else {
                    if input[x][y+1] != '.' {
                        unchecked.push((x, y+1));
                    }
                }
            }
            k += 1;
        }
        let mut r_fence = 0;
        for &(x, y) in region.iter() {
            input[x][y] = '.';
            r_fence += fence[x][y];
        }
        s += region.len()*r_fence;
    }
    s
}

fn task_two(mut input: Input) -> usize {
    let mut new_input = Vec::new();
    for i in 0..input.len() {
        let mut new_row = Vec::new();
        for j in 0..input[0].len() {
            new_row.push(input[i][j]);
            new_row.push(input[i][j]);
        }
        new_input.push(new_row.clone());
        new_input.push(new_row);
    }
    input = new_input;
    let mut fence = Vec::new();
    for i in 0..input.len() {
        let mut fence_row = Vec::new();
        for j in 0..input[i].len() {
            let c = input[i][j];
            let above = i == 0 || input[i-1][j] != c;
            let below = i + 1 == input.len() || input[i+1][j] != c;
            let left = j == 0 || input[i][j-1] != c;
            let right = j+1 == input[0].len() || input[i][j+1] != c;
            let nw = i == 0 || j == 0 || input[i-1][j-1] != c;
            let ne = i == 0 || j + 1 == input[0].len() || input[i-1][j+1] != c;
            let se = i + 1 == input.len() || j + 1 == input[0].len() || input[i+1][j+1] != c;
            let sw = i + 1 == input.len() || j == 0 || input[i+1][j-1] != c;
            let mut fences = 0;
            if above && right {
                fences += 1;
            }
            if right && below {
                fences += 1;
            }
            if below && left {
                fences += 1;
            }
            if left && above {
                fences += 1;
            }
            if !above && !below && !right && !left && (nw || sw || se || ne) {
                fences += 1;
            }
            fence_row.push(fences);
        }
        fence.push(fence_row);
    }
    let mut unchecked = Vec::from([(0,0)]);
    let mut s = 0;
    while let Some((i, j)) = unchecked.pop() {
        let c = input[i][j];
        if c == '.' {
            continue;
        }
        let mut region = vec![(i,j)];
        let mut k = 0;
        while k < region.len() {
            let (x, y) = region[k];
            if x > 0 {
                if input[x-1][y] == c  {
                    if !region.contains(&(x-1, y)) {
                        region.push((x-1, y));
                    }
                } else {
                    if input[x-1][y] != '.' {
                        unchecked.push((x-1, y));
                    }
                }
            }
            if x + 1 < input.len() {
                if input[x+1][y] == c  {
                    if !region.contains(&(x+1, y)) {
                        region.push((x+1, y));
                    }
                } else {
                    if input[x+1][y] != '.' {
                        unchecked.push((x+1, y));
                    }
                }
            }
            if y > 0 {
                if input[x][y-1] == c  {
                    if !region.contains(&(x, y-1)) {
                        region.push((x, y-1));
                    }
                } else {
                    if input[x][y-1] != '.' {
                        unchecked.push((x, y-1));
                    }
                }
            }
            if y + 1 < input[0].len() {
                if input[x][y+1] == c  {
                    if !region.contains(&(x, y+1)) {
                        region.push((x, y+1));
                    }
                } else {
                    if input[x][y+1] != '.' {
                        unchecked.push((x, y+1));
                    }
                }
            }
            k += 1;
        }
        let mut r_fence = 0;
        for &(x, y) in region.iter() {
            input[x][y] = '.';
            r_fence += fence[x][y];
        }
        s += region.len()*r_fence;
    }
    s/4
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
        assert_eq!(1930, task_one(input.clone()));
        assert_eq!(1206, task_two(input));
    }
}
