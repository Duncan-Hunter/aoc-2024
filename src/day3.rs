use std::collections::HashMap;

use regex::Regex;

use crate::util::read_data_from_file;

#[derive(PartialEq, Debug)]
struct Mul {
    left: isize,
    right: isize,
    start: usize,
}

fn de_corrupt(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let items = re
        .captures_iter(input)
        .map(|x| Mul {
            left: x[1].parse::<isize>().unwrap(),
            right: x[2].parse::<isize>().unwrap(),
            start: x.get(0).unwrap().start(),
        })
        .collect::<Vec<Mul>>();
    items
}

fn summed_mul(input: &str) -> isize {
    let items = de_corrupt(input);
    let sum: isize = items.iter().map(|x| x.left * x.right).sum();
    sum
}

pub fn part_1() -> () {
    let input = read_data_from_file("data/day3/puzzle.txt");
    let answer = summed_mul(&input);
    println!("{answer}")
}

fn combine(
    hay_start: usize,
    hay_end: usize,
    do_starts: &HashMap<usize, bool>,
    dont_starts: &HashMap<usize, bool>,
    muls: &HashMap<usize, Mul>,
) -> isize {
    let mut do_mul = true;
    let mut tot = 0;
    for i in hay_start..hay_end {
        if do_starts.contains_key(&i) {
            do_mul = true;
        } else if dont_starts.contains_key(&i) {
            do_mul = false;
        } else if muls.contains_key(&i) & do_mul {
            let mul = muls.get(&i).unwrap();
            tot += mul.left * mul.right;
        }
    }
    tot
}

fn do_summed_mul(input: &str) -> isize {
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don\'t\(\)").unwrap();

    let do_starts: HashMap<usize, bool> = do_re
        .captures_iter(input)
        .map(|x| (x.get(0).unwrap().start(), true))
        .collect::<HashMap<usize, bool>>();
    let dont_starts = dont_re
        .captures_iter(input)
        .map(|x| (x.get(0).unwrap().start(), false))
        .collect::<HashMap<usize, bool>>();
    let muls = de_corrupt(input)
        .into_iter()
        .map(|x| (x.start, x))
        .collect::<HashMap<usize, Mul>>();
    combine(0, input.len(), &do_starts, &dont_starts, &muls)
}

pub fn part_2() -> () {
    let input = read_data_from_file("data/day3/puzzle.txt");
    let result = do_summed_mul(&input);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let result =
            summed_mul("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }

    #[test]
    fn part_2_works() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = do_summed_mul(input);
        assert_eq!(result, 48);
    }
}
