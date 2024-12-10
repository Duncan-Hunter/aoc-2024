use std::collections::HashMap;

use crate::util::{input_to_grid, read_data_from_file};

fn find_trails(
    i: usize,
    j: usize,
    val: usize,
    grid: &Vec<Vec<usize>>,
    end_val: usize,
) -> HashMap<(usize, usize), usize> {
    if val == end_val {
        let mut set: HashMap<(usize, usize), usize> = HashMap::new();
        set.insert((i, j), 1);
        return set;
    }

    let mut result: HashMap<(usize, usize), usize> = HashMap::new();
    // check up
    if i > 0 {
        if *grid.get(i - 1).unwrap().get(j).unwrap() == val + 1 {
            let x = find_trails(i - 1, j, val + 1, grid, end_val);
            for ((end_i, end_j), v) in x {
                *result.entry((end_i, end_j)).or_insert(0) += v;
            }
        }
    }
    // check right
    if j < grid.get(0).unwrap().len() - 1 {
        if *grid.get(i).unwrap().get(j + 1).unwrap() == val + 1 {
            let x = find_trails(i, j + 1, val + 1, grid, end_val);
            for ((end_i, end_j), v) in x {
                *result.entry((end_i, end_j)).or_insert(0) += v;
            }
        }
    }
    // check down
    if i < grid.len() - 1 {
        if *grid.get(i + 1).unwrap().get(j).unwrap() == val + 1 {
            let x = find_trails(i + 1, j, val + 1, grid, end_val);
            for ((end_i, end_j), v) in x {
                *result.entry((end_i, end_j)).or_insert(0) += v;
            }
        }
    }
    // check left
    if j > 0 {
        if *grid.get(i).unwrap().get(j - 1).unwrap() == val + 1 {
            let x = find_trails(i, j - 1, val + 1, grid, end_val);
            for ((end_i, end_j), v) in x {
                *result.entry((end_i, end_j)).or_insert(0) += v;
            }
        }
    }
    result
}

fn trailheads(grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(&input_uri);
    let grid = input_to_grid::<usize>(&input);
    let trailheads = trailheads(&grid);
    let mut total: usize = 0;
    for trailhead in trailheads.iter() {
        let trail_ends = find_trails(trailhead.0, trailhead.1, 0, &grid, 9);
        total += trail_ends.len();
    }
    total
}

pub fn part_2(input_uri: &str) -> usize {
    let input = read_data_from_file(&input_uri);
    let grid = input_to_grid::<usize>(&input);
    let trailheads = trailheads(&grid);
    let mut total: usize = 0;
    for trailhead in trailheads.iter() {
        let trails = find_trails(trailhead.0, trailhead.1, 0, &grid, 9);
        for (_, trail_count) in trails {
            total += trail_count;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // let result = part_1("data/day10/test2.txt");
        let result = part_1("data/day10/test.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("data/day10/test.txt");
        assert_eq!(result, 81);
        let result = part_2("data/day10/test2.txt");
        assert_eq!(result, 3);
    }
}
