use crate::util::read_data_from_file;
use std::collections::HashMap;
use std::iter::zip;

pub fn part_1() -> () {
    let input_data = read_data_from_file("data/day1/puzzle1.txt");
    let input_data: Vec<&str> = input_data.split_ascii_whitespace().collect();
    let mut first_set: Vec<usize> = Vec::new();
    let mut second_set: Vec<usize> = Vec::new();
    for (i, entry) in input_data.iter().enumerate() {
        let entry = entry.trim().parse::<usize>().expect("Can't convert to usize");
        if i % 2 == 0 {
            first_set.push(entry);
        }
        else {
            second_set.push(entry);
        }
    }
    first_set.sort();
    second_set.sort();

    let mut tot = 0;
    for (entry_1, entry_2) in zip(first_set, second_set) {
        tot += entry_1.abs_diff(entry_2);
    }
    println!("{tot}");
}

pub fn part_2() -> () {
    let input_data = read_data_from_file("data/day1/puzzle1.txt");
    let input_data: Vec<usize> = input_data
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<usize>().expect("Can't convert"))
        .collect();

    let mut second_count: HashMap<usize, usize> = HashMap::new();

    for entry in input_data[1..].iter().step_by(2) {
        *second_count.entry(*entry).or_insert(0) += 1;
    }
    let mut tot = 0;
    for entry in input_data.iter().step_by(2) {
        tot += *second_count.entry(*entry).or_insert(0) * entry;
    }
    println!("{tot}");
}