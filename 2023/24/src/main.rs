#![allow(unused)]
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hail {
    fn intersects_plane(&self, other: &Self) -> (f64, f64) {
        let det = -self.vx as i64 * other.vy as i64 + self.vy as i64 * other.vx as i64;
        if det == 0 {
            //Antagelse: Baner har max ett skjæringspunkt
            assert!((self.x - other.x) * self.vx != (self.y - other.y) * self.vy);
            return (-1.0, -1.0);
        }
        let t_num = -other.vy * (other.x - self.x) + other.vx * (other.y - self.y);
        let t = t_num / det as f64;
        let s_num = -self.vy * (other.x - self.x) + self.vx * (other.y - self.y);
        let s = s_num / det as f64;
        (t, s)
    }
}

impl FromStr for Hail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" @ ").unwrap();
        let mut p = p.split(", ");
        let x = p.next().unwrap().parse().unwrap();
        let y = p.next().unwrap().parse().unwrap();
        let z = p.next().unwrap().parse().unwrap();

        let mut v = v.split(", ");
        let vx = v.next().unwrap().parse().unwrap();
        let vy = v.next().unwrap().parse().unwrap();
        let vz = v.next().unwrap().parse().unwrap();

        Ok(Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        })
    }
}
impl Display for Hail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.x, self.y, self.z, self.vx, self.vy, self.vz
        )
    }
}

type Input = Vec<Hail>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn task_one(input: Input, start: f64, stop: f64) -> i32 {
    let mut summation = 0;
    for j in 0..input.len() {
        for i in 0..j {
            let h = input[i];
            let k = input[j];
            let (t, s) = h.intersects_plane(&k);
            let h_imp_x = h.x + h.vx * t;
            let h_imp_y = h.y + h.vy * t;
            let k_imp_x = k.x + k.vx * s;
            let k_imp_y = k.y + k.vy * s;
            if t < 0.0
                || s < 0.0
                || h_imp_x < start
                || h_imp_y < start
                || h_imp_x > stop
                || h_imp_y > stop
            {
                continue;
            }
            summation += 1;
        }
    }
    summation
}

fn calc_error(input: &Input, h: &Hail) -> f64 {
    let mut e: f64 = 0.0;
    for k in input {
        let diff = Hail {
            x: h.x - k.x,
            y: h.y - k.y,
            z: h.z - k.z,
            vx: h.vx - k.vx,
            vy: h.vy - k.vy,
            vz: h.vz - k.vz,
        };
        let v_length = (diff.vx.powi(2) + diff.vy.powi(2) + diff.vz.powi(2));
        let dot_prod = (diff.x * diff.vx + diff.y * diff.vy + diff.z * diff.vz);
        e += (diff.x - dot_prod * diff.vx / v_length).powi(2)
            + (diff.y - dot_prod * diff.vy / v_length).powi(2)
            + (diff.z - dot_prod * diff.vz / v_length).powi(2);
    }
    e
}

fn line_dist(input: &Input, h: &Hail) -> f64 {
    0.0
}

fn task_two(input: Input) -> f64 {
    0.0
}

fn main() {
    let input = read_input("input.txt");
    println!(
        "Task one: {}",
        task_one(input.clone(), 200_000_000_000_000.0, 400_000_000_000_000.0)
    );
    println!("Task two: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        let input = read_input("example_input.txt");
        assert_eq!(2, task_one(input.clone(), 7.0, 27.0));
        assert_eq!(47.0, task_two(input));
    }
}
