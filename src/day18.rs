use crate::util::{min_in_hashset, next_xy, read_data_from_file};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

fn find_shortest_path(
    start: (usize, usize),
    end: (usize, usize),
    prev: HashMap<(usize, usize), (usize, usize)>,
) -> Vec<(usize, usize)> {
    if end == start {
        return vec![start];
    }
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut current = end;
    while prev.contains_key(&current) {
        result.insert(0, current);
        current = *prev.get(&current).unwrap();
    }
    result
}

fn find_neighbours(
    current: (usize, usize),
    end: (usize, usize),
    visited: &HashSet<(usize, usize)>,
    maze: &HashMap<(usize, usize), char>,
) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    if current == end {
        return neighbours;
    }
    for direction in "^><v".chars() {
        match next_xy(current.0, current.1, direction) {
            Some((next_i, next_j)) => {
                if (!visited.contains(&(next_i, next_j)))
                    & (*maze.get(&(next_i, next_j)).unwrap() != '#')
                {
                    neighbours.push((next_i, next_j));
                }
            }
            None => {}
        }
    }
    neighbours
}

fn dijktra_short_path(
    start: (usize, usize),
    end: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
) -> (
    HashMap<(usize, usize), usize>,
    HashMap<(usize, usize), (usize, usize)>,
) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: HashSet<(usize, usize)> = HashSet::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    queue.insert(start);
    dist.insert(start, 0);

    while !queue.is_empty() {
        let current = min_in_hashset::<(usize, usize)>(&queue, &mut dist);
        visited.insert(current);
        queue.remove(&current);
        let neighbours = find_neighbours(current, end, &visited, maze);
        for neighbour in neighbours {
            let alt = dist.get(&current).unwrap() + 1;
            if alt < *dist.entry(neighbour).or_insert(usize::MAX) {
                dist.insert(neighbour, alt);
                prev.insert(neighbour, current);
            }
            queue.insert(neighbour);
        }
    }
    (dist, prev)
}

fn create_maze(
    blocks: &Vec<(usize, usize)>,
    grid_size: (usize, usize),
) -> HashMap<(usize, usize), char> {
    let mut maze: HashMap<(usize, usize), char> = HashMap::new();
    for i in 1..grid_size.0 + 1 {
        for j in 1..grid_size.1 + 1 {
            maze.insert((i, j), '.');
        }
    }
    for i in 0..grid_size.0 + 2 {
        maze.insert((i, 0), '#');
        maze.insert((i, grid_size.1 + 1), '#');
    }
    for j in 0..grid_size.1 + 2 {
        maze.insert((0, j), '#');
        maze.insert((grid_size.0 + 1, j), '#');
    }
    for (i, j) in blocks {
        maze.insert((j + 1, i + 1), '#');
    }
    return maze;
}

fn process_input(input: &str) -> Vec<(usize, usize)> {
    let pattern = Regex::new(r"(\d+),(\d+)").unwrap();

    pattern
        .captures_iter(input)
        .map(|x| {
            let (_, [x, y]) = x.extract();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<(usize, usize)>>()
}

fn shortest_path_len(
    blocks: &Vec<(usize, usize)>,
    grid_size: (usize, usize),
    num_blocks: usize,
) -> usize {
    let first_blocks = &blocks[..num_blocks].to_vec();
    let grid_size = (grid_size.0 + 1, grid_size.1 + 1);
    let maze = create_maze(first_blocks, grid_size);
    let start = (1usize, 1usize);
    let end = grid_size;
    let (_, prev) = dijktra_short_path(start, end, &maze);
    let shortest_path = find_shortest_path(start, end, prev);
    shortest_path.len()
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let blocks = process_input(&input);
    shortest_path_len(&blocks, (70, 70), 1024)
}

fn place_block(maze: &mut HashMap<(usize, usize), char>, i: usize, j: usize) {
    maze.insert((i, j), '#');
}

fn find_not_possible(
    blocks: &Vec<(usize, usize)>,
    grid_size: (usize, usize),
) -> Option<(usize, usize)> {
    let no_blocks: Vec<(usize, usize)> = vec![];
    let grid_size = (grid_size.0 + 1, grid_size.1 + 1);
    let mut maze = create_maze(&no_blocks, grid_size);
    let start = (1usize, 1usize);
    let end = grid_size;
    let (_, mut prev) = dijktra_short_path(start, end, &maze);
    let mut shortest_path = find_shortest_path(start, end, prev);
    let mut shortest_path_set = shortest_path
        .into_iter()
        .collect::<HashSet<(usize, usize)>>();
    for (x, y) in blocks.iter() {
        place_block(&mut maze, y + 1, x + 1);
        if shortest_path_set.contains(&(y + 1, x + 1)) {
            (_, prev) = dijktra_short_path(start, end, &maze);
            shortest_path = find_shortest_path(start, end, prev);
            if shortest_path.len() == 0 {
                return Some((*x, *y));
            }
            shortest_path_set = shortest_path
                .into_iter()
                .collect::<HashSet<(usize, usize)>>();
        }
    }
    None
}

pub fn part_2(input_uri: &str) -> (usize, usize) {
    let input = read_data_from_file(input_uri);
    let blocks = process_input(&input);
    find_not_possible(&blocks, (70, 70)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_data_from_file("data/day18/test.txt");
        let blocks = process_input(&input);
        let result = shortest_path_len(&blocks, (6, 6), 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_2() {
        let input = read_data_from_file("data/day18/test.txt");
        let blocks = process_input(&input);
        let answer = find_not_possible(&blocks, (6, 6));
        assert_eq!(answer, Some((6, 1)))
    }
}
