use std::collections::HashMap;

use crate::util::read_data_from_file;

fn split_stone(stone: &str) -> (usize, usize) {
    let n_digits = stone.chars().count();
    if n_digits % 2 != 0 {
        panic!("Only call this function with even number of chars")
    }
    let mid_point = n_digits / 2;
    let stone_1 = stone.get(..mid_point).unwrap().parse::<usize>().unwrap();
    let stone_2 = stone.get(mid_point..).unwrap().parse::<usize>().unwrap();
    (stone_1, stone_2)
}

fn blink(input: &Vec<usize>) -> Vec<usize> {
    let mut new_list: Vec<usize> = Vec::new();
    for stone in input.iter() {
        let stone_str = stone.to_string();
        if *stone == 0 {
            new_list.push(1);
        } else if stone_str.chars().count() % 2 == 0 {
            let new_stones = split_stone(&stone_str);
            new_list.push(new_stones.0);
            new_list.push(new_stones.1);
        } else {
            new_list.push(stone * 2024);
        }
    }
    new_list
}

fn process_input(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<usize>().expect("Can't convert to usize"))
        .collect::<Vec<usize>>()
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let mut stones = process_input(&input);
    for _ in 0..25 {
        stones = blink(&stones)
    }
    stones.len()
}

fn blink_stone(stone: usize) -> Vec<usize> {
    let mut new_stones: Vec<usize> = Vec::new();
    let stone_str = stone.to_string();
    if stone == 0 {
        new_stones.push(1);
    } else if stone_str.chars().count() % 2 == 0 {
        let split_stones = split_stone(&stone_str);
        new_stones.push(split_stones.0);
        new_stones.push(split_stones.1);
    } else {
        new_stones.push(stone * 2024);
    }
    new_stones
}

fn split_stone_recursive(
    stone: usize,
    current_blink: usize,
    total_blinks: usize,
    stone_count_cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let levels_to_go = total_blinks - current_blink;
    match stone_count_cache.get(&(stone, levels_to_go)) {
        Some(a) => return *a,
        None => {}
    }
    if current_blink == total_blinks - 1 {
        return blink_stone(stone).len();
    }

    let next_blink = blink_stone(stone);

    let mut count_after_blink: usize = 0;
    for s in next_blink {
        count_after_blink +=
            split_stone_recursive(s, current_blink + 1, total_blinks, stone_count_cache)
    }
    stone_count_cache.insert((stone, levels_to_go), count_after_blink);
    count_after_blink
}

pub fn part_2(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let stones = process_input(&input);
    let mut result: usize = 0;
    let mut stone_level_cache: HashMap<(usize, usize), usize> = HashMap::new();
    for stone in stones {
        result += split_stone_recursive(stone, 0, 75, &mut stone_level_cache)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_stone() {
        let result = split_stone("6152");
        assert_eq!(result.0, 61);
        assert_eq!(result.1, 52);
        let result = split_stone("612523");
        assert_eq!(result.0, 612);
        assert_eq!(result.1, 523);
    }

    #[test]
    fn test_blink() {
        let input = read_data_from_file("data/day11/test2.txt");
        let mut stones = process_input(&input);
        stones = blink(&stones);
        assert_eq!(stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_part_1() {
        // let result = part_1("data/day10/test2.txt");
        let result = part_1("data/day11/test.txt");
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_split_stone_recursive() {
        let input = read_data_from_file("data/day11/test.txt");
        let stones: Vec<usize> = process_input(&input);
        let mut result: usize = 0;
        let mut stone_level_cache: HashMap<(usize, usize), usize> = HashMap::new();
        for stone in stones {
            result += split_stone_recursive(stone, 0, 6, &mut stone_level_cache)
        }
        assert_eq!(result, 22);
        let stones: Vec<usize> = process_input(&input);
        let mut result: usize = 0;
        let mut stone_level_cache: HashMap<(usize, usize), usize> = HashMap::new();
        for stone in stones {
            result += split_stone_recursive(stone, 0, 25, &mut stone_level_cache)
        }
        assert_eq!(result, 55312);
    }
}
