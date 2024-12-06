use std::collections::HashSet;

use crate::util::read_data_from_file;

fn sum_x(array: &Vec<Vec<char>>) -> usize {
    let mut tot = 0;
    for line in array {
        for ch in line {
            if *ch == 'X' {
                tot += 1
            }
        }
    }
    tot
}

fn x_inds(array: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut inds: Vec<(usize, usize)> = Vec::new();
    for (i, line) in array.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == 'X' {
                inds.push((i, j));
            }
        }
    }
    inds
}

fn next_char(
    array: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    direction: &str,
) -> Option<(usize, usize, char)> {
    if direction == "up" {
        let next_row = row.checked_add_signed(-1)?; // if None, we're leaving
        return Some((next_row, col, *array.get(next_row)?.get(col)?));
    } else if direction == "right" {
        let next_col = col + 1;
        return Some((row, next_col, *array.get(row)?.get(next_col)?)); // If None, we're leaving
    } else if direction == "down" {
        let next_row = row + 1;
        return Some((next_row, col, *array.get(next_row)?.get(col)?));
    } else {
        let next_col = col.checked_add_signed(-1)?;
        return Some((row, next_col, *array.get(row)?.get(next_col)?));
    }
}

fn iterate_path(
    array: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    start_direction: &str,
) -> Option<Vec<Vec<char>>> {
    let mut row = start_row;
    let mut col = start_col;
    let mut direction = start_direction;

    let mut in_bounds = true;
    let mut answer_array = array.clone();

    let mut seen: HashSet<(usize, usize, &str)> = HashSet::new();

    while in_bounds {
        match next_char(array, row, col, direction) {
            Some((next_row, next_col, ch)) => {
                if ch == '#' {
                    direction = match direction {
                        "up" => "right",
                        "right" => "down",
                        "down" => "left",
                        "left" => "up",
                        _ => panic!("unknown direction"),
                    }
                } else {
                    answer_array[row][col] = 'X'; // if we move off, set to 'X'
                    seen.insert((row, col, direction));
                    row = next_row;
                    col = next_col;
                }
                // cases where we've seen it, but it's not a loop?
                if seen.contains(&(row, col, direction)) {
                    return None;
                }
            }
            None => {
                answer_array[row][col] = 'X'; // if we leave, set to 'X'
                in_bounds = false;
            }
        }
    }
    Some(answer_array)
}

fn find_starting_location(array: &Vec<Vec<char>>) -> (usize, usize, &str) {
    for (i, row) in array.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                return (i, j, "up");
            } else if *ch == '>' {
                return (i, j, "right");
            } else if *ch == 'v' {
                return (i, j, "down");
            } else if *ch == '<' {
                return (i, j, "left");
            }
        }
    }
    panic!("Can't find the starting location")
}

fn input_to_array(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn part_1() -> () {
    let input = read_data_from_file("data/day6/puzzle.txt");
    let array = input_to_array(&input);
    let (row_num, col_num, direction) = find_starting_location(&array);
    let answer_array = iterate_path(&array, row_num, col_num, direction);
    match answer_array {
        Some(a) => {
            let answer = sum_x(&a);
            println!("{answer}");
        }
        None => (),
    }
}

fn find_loops(array: &Vec<Vec<char>>) -> usize {
    let (row_num, col_num, direction) = find_starting_location(&array);
    let original_path: Vec<Vec<char>> = iterate_path(&array, row_num, col_num, direction).unwrap();
    let original_path_inds = x_inds(&original_path);
    let mut count: usize = 0;
    for (i, j) in original_path_inds {
        if (i, j) == (row_num, col_num) {
            continue;
        }
        let mut obstacle_array = array.clone();
        obstacle_array[i][j] = '#';
        match iterate_path(&obstacle_array, row_num, col_num, direction) {
            Some(_) => continue,
            None => count += 1,
        }
    }
    count
}

pub fn part_2() -> () {
    // add obstacles, somewhere, test if we're in a loop, and count the numbers of options
    // very tempted to brute force this
    let input = read_data_from_file("data/day6/puzzle.txt");
    let array = input_to_array(&input);
    let count = find_loops(&array);
    println!("{count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // assert_eq!
        let input = read_data_from_file("data/day6/test.txt");
        let array = input_to_array(&input);
        assert_eq!(array.len(), 10);
        assert_eq!(array[0].len(), 10);
        assert_eq!(array[6][4], '^');
        let (row_num, col_num, direction) = find_starting_location(&array);
        assert_eq!(row_num, 6);
        assert_eq!(col_num, 4);
        assert_eq!(direction, "up");
        let answer_array = iterate_path(&array, row_num, col_num, direction);
        match answer_array {
            Some(a) => {
                let answer = sum_x(&a);
                assert_eq!(answer, 41);
            }
            None => (),
        }
    }

    #[test]
    fn part_2_works() {
        let input = read_data_from_file("data/day6/test.txt");
        let array = input_to_array(&input);
        let answer = find_loops(&array);
        assert_eq!(answer, 6);
    }
}
