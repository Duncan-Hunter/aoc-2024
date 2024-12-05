use crate::util::read_data_from_file;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

// can't use Kahn's algorithm as there are cycles. Wasted a lot of time on that.

fn process_input(input: &str) -> (usize, usize) {
    let mut pages: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut is_instruction = true;
    let mut total = 0;
    let mut bad_total = 0;
    for line in input.lines() {
        if line.len() == 0 {
            is_instruction = false;
        } else if is_instruction {
            let numbers = line
                .split('|')
                .map(|x| x.trim().parse::<usize>().expect("Can't parse"))
                .collect::<Vec<usize>>();
            let number = *numbers.get(0).unwrap();
            let before = *numbers.get(1).unwrap();
            let befores = pages.entry(number).or_insert(HashSet::new());
            befores.insert(before);
        } else {
            let mut numbers = line
                .split(',')
                .map(|x| x.trim().parse::<usize>().expect("Can't parse"))
                .collect::<Vec<usize>>();
            let mut good = true;
            for window in numbers.windows(2) {
                let first = *window.get(0).unwrap();
                let second = *window.get(1).unwrap();
                let befores = pages.entry(second).or_default();
                if befores.contains(&first) {
                    good = false;
                    break;
                }
            }
            if good {
                let middle_ind = numbers.len() / 2;
                total += numbers.get(middle_ind).unwrap();
            } else {
                numbers.sort_by(|a, b| {
                    if pages.entry(*b).or_default().contains(a) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });
                bad_total += numbers.get(numbers.len() / 2).unwrap();
            }
        }
    }

    (total, bad_total)
}

pub fn part_1() -> () {
    let input = read_data_from_file("data/day5/puzzle.txt");
    let (good_answer, _) = process_input(&input);
    println!("{good_answer}");
}

pub fn part_2() -> () {
    let input = read_data_from_file("data/day5/puzzle.txt");
    let (_, bad_answer) = process_input(&input);
    println!("{bad_answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // assert_eq!
        let input = read_data_from_file("data/day5/test.txt");
        let (good_answer, _) = process_input(&input);
        assert_eq!(good_answer, 143);
    }

    #[test]
    fn part_2_works() {
        let input = read_data_from_file("data/day5/test.txt");
        let (_, bad_answer) = process_input(&input);
        assert_eq!(bad_answer, 123);
    }
}
