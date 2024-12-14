use std::collections::HashMap;
use std::io;

use crate::util::read_data_from_file;
use regex::Regex;

fn wrapped_next_location(
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    grid_width: usize,
    grid_height: usize,
) -> (isize, isize) {
    let mut new_x = x + dx;
    let mut new_y = y + dy;
    new_x = new_x % grid_width as isize;
    new_y = new_y % grid_height as isize;
    if new_x < 0 {
        new_x = grid_width as isize + new_x;
    }
    if new_y < 0 {
        new_y = grid_height as isize + new_y;
    }
    (new_x, new_y)
}

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn non_blocking_steps(&mut self, n_steps: usize, grid_width: usize, grid_height: usize) {
        let dx = self.vx * n_steps as isize;
        let dy = self.vy * n_steps as isize;
        let (new_x, new_y) = wrapped_next_location(self.x, self.y, dx, dy, grid_width, grid_height);
        self.x = new_x;
        self.y = new_y;
    }
}

fn process_input(input: &str) -> Vec<Robot> {
    // for line in input.lines()
    //p=99,12 v=19,18
    let mut robots: Vec<Robot> = Vec::new();
    let pattern = Regex::new(r"p\=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for (_, [x, y, vx, vy]) in pattern.captures_iter(input).map(|c| c.extract()) {
        let x = x.parse::<isize>().expect("Can't convert to isize");
        let y = y.parse::<isize>().expect("Can't convert to isize");
        let vx = vx.parse::<isize>().expect("Can't convert to isize");
        let vy = vy.parse::<isize>().expect("Can't convert to isize");
        robots.push(Robot { x, y, vx, vy });
    }
    robots
}

fn count(robots: &mut Vec<Robot>, grid_height: usize, grid_width: usize) -> usize {
    let mut quadrant_count: HashMap<(isize, isize), usize> = HashMap::new();
    let half_grid_width = (grid_width / 2) as isize;
    let half_grid_height = (grid_height / 2) as isize;
    for robot in robots.iter() {
        if (robot.x == half_grid_width) | (robot.y == half_grid_height) {
            continue;
        }
        let quad_x = robot.x / (half_grid_width + 1);
        let quad_y = robot.y / (half_grid_height + 1);
        *quadrant_count.entry((quad_x, quad_y)).or_insert(0) += 1;
    }
    let mut safety_factor = 1;
    for (_, count) in quadrant_count {
        safety_factor *= count;
    }
    safety_factor
}

fn step_and_count(
    robots: &mut Vec<Robot>,
    n_steps: usize,
    grid_height: usize,
    grid_width: usize,
) -> usize {
    for robot in robots.iter_mut() {
        robot.non_blocking_steps(n_steps, grid_width, grid_height);
    }

    count(robots, grid_height, grid_width)
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let mut robots = process_input(&input);
    let grid_height: usize = 103;
    let grid_width: usize = 101;
    let n_steps = 100;
    let safety_factor = step_and_count(&mut robots, n_steps, grid_height, grid_width);
    safety_factor
}

fn grid(robots: &Vec<Robot>, grid_height: usize, grid_width: usize) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..grid_width + 2 {
        grid.push(Vec::new());
        for _ in 0..grid_height + 2 {
            grid[i].push(' ')
        }
    }
    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '▃';
    }
    grid
}

fn check_line(robots: &Vec<Robot>, grid_height: usize, grid_width: usize) -> bool {
    let grid = grid(robots, grid_height, grid_width);

    for line in grid {
        let line_count = line
            .iter()
            .map(|x| if *x == '▃' { 1 } else { 0 })
            .sum::<usize>();
        if line_count >= 30 {
            return true;
        }
    }
    return false;
}

fn display_grid(robots: &Vec<Robot>, grid_height: usize, grid_width: usize) {
    let grid = grid(robots, grid_height, grid_width);

    for line in grid {
        let s: String = line.into_iter().collect();
        println!("{s}");
    }
}

pub fn part_2(input_uri: &str) {
    let input = read_data_from_file(input_uri);
    let mut robots = process_input(&input);
    let grid_height: usize = 103;
    let grid_width: usize = 101;
    let mut user_str = String::new();
    let mut n_steps: usize = 0;
    loop {
        if check_line(&robots, grid_height, grid_width) {
            println!("{n_steps}");
            display_grid(&robots, grid_height, grid_width);
            io::stdin()
                .read_line(&mut user_str)
                .expect("Failed to read line");
        }
        step_and_count(&mut robots, 1, grid_height, grid_width);
        n_steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapped_next_location() {
        let result = wrapped_next_location(1, 1, -2, 2, 3, 3);
        assert_eq!(result, (2, 0));
        let result = wrapped_next_location(1, 1, 2, -3, 5, 3);
        assert_eq!(result, (3, 1));
        let result = wrapped_next_location(1, 1, 2, -9, 5, 3);
        assert_eq!(result, (3, 1));
    }

    #[test]
    fn test_step_and_count() {
        let input = read_data_from_file("data/day14/test.txt");
        let mut robots = process_input(&input);
        let grid_height: usize = 7;
        let grid_width: usize = 11;
        let n_steps = 100;
        let safety_factor = step_and_count(&mut robots, n_steps, grid_height, grid_width);
        assert_eq!(safety_factor, 12);
    }
}
