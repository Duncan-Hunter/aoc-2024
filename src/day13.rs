use crate::util::read_data_from_file;
use regex::Regex;

const TOL: f64 = 40.0 - 39.999;

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

impl Machine {
    fn solve(&self) -> Option<Vec<f64>> {
        let mut lines = vec![
            vec![self.a.0, self.b.0, self.prize.0],
            vec![self.a.1, self.b.1, self.prize.1],
        ];
        let height = lines.len();
        let width = lines[0].len();
        for i in 0..height {
            for j in i + 1..height {
                let factor = lines[j][i] / lines[i][i];
                for k in 0..width {
                    lines[j][k] = lines[j][k] - factor * lines[i][k];
                }
            }
        }
        for i in (0..height).rev() {
            for j in (0..i).rev() {
                let factor = lines[j][i] / lines[i][i];
                for k in 0..width {
                    lines[j][k] = lines[j][k] - factor * lines[i][k];
                }
            }
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..height {
            let x = lines[i][width - 1] / lines[i][i];
            if (x - x.round()).abs() <= TOL {
                result.push(x.round());
            } else {
                dbg!((x - x.round()).abs());
                return None;
            }
        }
        Some(result)
    }
}

fn process_input(input: &str, prize_mod: f64) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut a: (f64, f64) = (0.0, 0.0);
    let mut b: (f64, f64) = (0.0, 0.0);
    let mut prize: (f64, f64) = (0.0, 0.0);
    let a_pattern = Regex::new(r"A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_pattern = Regex::new(r"B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"X\=(\d+), Y\=(\d+)").unwrap();

    for line in input.lines() {
        for (_, [x, y]) in a_pattern.captures_iter(line).map(|c| c.extract()) {
            a = (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        }
        for (_, [x, y]) in b_pattern.captures_iter(line).map(|c| c.extract()) {
            b = (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        }
        for (_, [x, y]) in prize_pattern.captures_iter(line).map(|c| c.extract()) {
            prize = (
                x.parse::<f64>().unwrap() + prize_mod,
                y.parse::<f64>().unwrap() + prize_mod,
            )
        }

        if line.is_empty() {
            let machine = Machine { a, b, prize };
            machines.push(machine);
        }
    }
    let machine = Machine { a, b, prize };
    machines.push(machine);
    machines
}

pub fn part_1(input_uri: &str) -> f64 {
    let input = read_data_from_file(input_uri);
    let machines = process_input(&input, 0.0);
    let mut total_cost: f64 = 0.0;
    for machine in machines {
        match machine.solve() {
            Some(x) => {
                total_cost += x[0] * 3.0;
                total_cost += x[1] * 1.0;
            }
            None => {}
        }
    }
    total_cost
}

pub fn part_2(input_uri: &str) -> f64 {
    let input = read_data_from_file(input_uri);
    let machines = process_input(&input, 10000000000000.0);
    let mut total_cost: f64 = 0.0;
    for machine in machines {
        match machine.solve() {
            Some(x) => {
                total_cost += x[0] * 3.0;
                total_cost += x[1] * 1.0;
                dbg!(x);
            }
            None => {
                println!("No solution")
            }
        }
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("data/day13/test.txt");
        assert_eq!(result, 480.0);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("data/day13/test.txt");
        dbg!(result);
    }
}
