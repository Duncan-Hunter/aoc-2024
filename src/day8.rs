use std::collections::{HashMap, HashSet};

use crate::util::read_data_from_file;

fn in_bounds(i: isize, j: isize, height: isize, width: isize) -> bool {
    if (i >= 0) & (i < height) & (j >= 0) & (j < width) {
        return true;
    }
    false
}

fn char_locations(grid: Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch != '.' {
                let locs = locations.entry(*ch).or_insert(Vec::new());
                locs.push((i as isize, j as isize));
            }
        }
    }
    locations
}

fn input_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn part_1(input: &str) -> usize {
    let input = read_data_from_file(input);
    let grid = input_to_grid(&input);
    let (height, width) = (grid.len() as isize, grid[0].len() as isize);
    let antenna_locations = char_locations(grid);
    // iterate over each antenna type
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (_, locations) in antenna_locations.iter() {
        let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        for (base_i, base_j) in locations.iter() {
            for (next_i, next_j) in locations.iter() {
                if (next_i == base_i) & (next_j == base_j) {
                    continue;
                }
                if visited.contains(&(*next_i, *next_j, *base_i, *base_j)) {
                    continue;
                }
                let dx = next_i - base_i;
                let dy = next_j - base_j;
                let antinode_1 = (base_i - dx, base_j - dy);
                let antinode_2 = (next_i + dx, next_j + dy);
                if in_bounds(antinode_1.0, antinode_1.1, height, width) {
                    antinodes.insert(antinode_1);
                }
                if in_bounds(antinode_2.0, antinode_2.1, height, width) {
                    antinodes.insert(antinode_2);
                }
                visited.insert((*base_i, *base_j, *next_i, *next_j));
            }
        }
    }
    antinodes.len()
}

pub fn part_2(input: &str) -> usize {
    let input = read_data_from_file(input);
    let grid = input_to_grid(&input);
    let (height, width) = (grid.len() as isize, grid[0].len() as isize);
    let antenna_locations = char_locations(grid);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (_, locations) in antenna_locations.iter() {
        let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        for (base_i, base_j) in locations.iter() {
            for (next_i, next_j) in locations.iter() {
                if (next_i == base_i) & (next_j == base_j) {
                    continue;
                }
                if visited.contains(&(*next_i, *next_j, *base_i, *base_j)) {
                    continue;
                }
                let dx = next_i - base_i;
                let dy = next_j - base_j;
                let mut antinode = (*base_i, *base_j);
                while in_bounds(antinode.0, antinode.1, height, width) {
                    antinodes.insert((antinode.0, antinode.1));
                    antinode = (antinode.0 - dx, antinode.1 - dy);
                }
                let mut antinode = (*base_i, *base_j);
                while in_bounds(antinode.0, antinode.1, height, width) {
                    antinodes.insert((antinode.0, antinode.1));
                    antinode = (antinode.0 + dx, antinode.1 + dy);
                }
                visited.insert((*base_i, *base_j, *next_i, *next_j));
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let answer = part_1("data/day8/test.txt");
        assert_eq!(answer, 14);
        let answer = part_1("data/day8/test2.txt");
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2("data/day8/test.txt");
        assert_eq!(answer, 34);
        let answer = part_2("data/day8/test3.txt");
        assert_eq!(answer, 9);
    }
}
