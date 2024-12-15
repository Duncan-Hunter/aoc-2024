use crate::util::{input_to_grid_map, read_data_from_file};
use core::panic;
use std::collections::HashMap;

fn score(grid: &HashMap<(usize, usize), char>, score_char: char) -> usize {
    let mut score: usize = 0;
    for ((x, y), ch) in grid {
        if *ch == score_char {
            score += 100 * x + y
        }
    }
    score
}

fn move_recursive(
    x: usize,
    y: usize,
    instruction: (isize, isize),
    grid: &mut HashMap<(usize, usize), char>,
) {
    let this_char = *grid.get(&(x, y)).unwrap();
    let next_x = x.checked_add_signed(instruction.0).unwrap();
    let next_y = y.checked_add_signed(instruction.1).unwrap();
    let next_char = *grid.get(&(next_x, next_y)).unwrap();
    match (next_char, next_x != x) {
        ('[', true) => {
            move_recursive(next_x, next_y, instruction, grid);
            move_recursive(next_x, next_y + 1, instruction, grid);
        }
        (']', true) => {
            move_recursive(next_x, next_y, instruction, grid);
            move_recursive(next_x, next_y - 1, instruction, grid);
        }
        ('#', false) => {
            panic!("Shouldn't be moving me");
        }
        ('#', true) => {
            panic!("Shouldn't be moving me");
        }
        ('.', true) => {}
        ('.', false) => {}
        _ => {
            move_recursive(next_x, next_y, instruction, grid);
        }
    }
    grid.insert((next_x, next_y), this_char);
    grid.insert((x, y), '.');
}

fn check_moveable(
    x: usize,
    y: usize,
    instruction: (isize, isize),
    grid: &HashMap<(usize, usize), char>,
) -> bool {
    let this_char = *grid.get(&(x, y)).unwrap();
    if this_char == '.' {
        return true;
    } else if this_char == '#' {
        return false;
    }
    let next_x = x.checked_add_signed(instruction.0).unwrap();
    let next_y = y.checked_add_signed(instruction.1).unwrap();

    let moveable = match (this_char, next_x != x) {
        ('[', true) => {
            //direction
            let extra_side_y = next_y + 1;
            let this_moveable = check_moveable(next_x, next_y, instruction, grid);
            let extra_moveable = check_moveable(next_x, extra_side_y, instruction, grid);
            this_moveable & extra_moveable
        }
        (']', true) => {
            let extra_side_y = next_y - 1;
            let this_moveable = check_moveable(next_x, next_y, instruction, grid);
            let extra_moveable = check_moveable(next_x, extra_side_y, instruction, grid);
            this_moveable & extra_moveable
        }
        _ => check_moveable(next_x, next_y, instruction, grid),
    };
    moveable
}

fn process_instruction(
    instruction: char,
    robot_location: (usize, usize),
    grid: &mut HashMap<(usize, usize), char>,
) -> (usize, usize) {
    let instruction: (isize, isize) = match instruction {
        '>' => (0, 1),
        '^' => (-1, 0),
        '<' => (0, -1),
        'v' => (1, 0),
        _ => panic!("Unknown instruction"),
    };
    if check_moveable(robot_location.0, robot_location.1, instruction, grid) {
        move_recursive(robot_location.0, robot_location.1, instruction, grid);
        return (
            robot_location.0.checked_add_signed(instruction.0).unwrap(),
            robot_location.1.checked_add_signed(instruction.1).unwrap(),
        );
    }
    robot_location
}

fn find_robot(grid: &HashMap<(usize, usize), char>) -> (usize, usize) {
    for ((i, j), ch) in grid {
        if *ch == '@' {
            return (*i, *j);
        }
    }
    panic!("Can't find the robot :(")
}

fn process_input(input: &str) -> (HashMap<(usize, usize), char>, Vec<char>) {
    let (grid, instructions) = match input.split_once("\n\n") {
        Some(v) => v,
        None => panic!("Can't split into grid and instructions"),
    };
    let grid: HashMap<(usize, usize), char> = input_to_grid_map(grid);
    let instructions: Vec<char> = instructions.lines().flat_map(|s| s.chars()).collect();
    (grid, instructions)
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let (mut grid, instructions) = process_input(&input);
    let mut robot_location = find_robot(&grid);
    for instruction in instructions {
        robot_location = process_instruction(instruction, robot_location, &mut grid);
    }
    score(&grid, 'O')
}

fn double_width(grid: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    let mut double_width_grid = HashMap::new();
    for ((x, y), ch) in grid {
        let j = y * 2;
        let chars = match *ch {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            '@' => ('@', '.'),
            _ => panic!("Unknown character"),
        };
        double_width_grid.insert((*x, j), chars.0);
        double_width_grid.insert((*x, j + 1), chars.1);
    }
    double_width_grid
}

pub fn part_2(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let (grid, instructions) = process_input(&input);
    let mut double_grid = double_width(&grid);
    let mut robot_location = find_robot(&double_grid);
    for instruction in instructions {
        robot_location = process_instruction(instruction, robot_location, &mut double_grid);
    }
    score(&double_grid, '[')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("data/day15/test.txt");
        assert_eq!(result, 10092);
    }

    // build some test cases
    #[test]
    fn test_input() {
        let input = read_data_from_file("data/day15/puzzle.txt");
        let (grid, instructions) = process_input(&input);
        let mut double_grid = double_width(&grid);
        let pre_count = double_grid.iter().filter(|(_, ch)| **ch == '[').count();
        let mut robot_location = find_robot(&double_grid);
        for instruction in instructions {
            robot_location = process_instruction(instruction, robot_location, &mut double_grid);
        }
        let post_count = double_grid.iter().filter(|(_, ch)| **ch == '[').count();
        assert_eq!(pre_count, post_count);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("data/day15/test.txt");
        assert_eq!(result, 9021);
        let result = part_2("data/day15/test3.txt");
        assert_eq!(result, 105 + 207 + 306);
        let result = part_2("data/day15/test4.txt");
        assert_eq!(result, 814);
        let result = part_2("data/day15/test5.txt");
        assert_eq!(result, 1220);
        let result = part_2("data/day15/test6.txt");
        assert_eq!(result, 1218);
        let result = part_2("data/day15/test7.txt");
        assert_eq!(result, 1012);
        let result = part_2("data/day15/test8.txt");
        assert_eq!(result, 1021);
        let result = part_2("data/day15/test9.txt");
        assert_eq!(result, 1024);
        let result = part_2("data/day15/test10.txt");
        assert_eq!(result, 1021);
    }
}
