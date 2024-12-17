use crate::util::{find_char, input_to_grid_map, next_xy, read_data_from_file};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

fn score(path: &Vec<(usize, usize, char)>) -> usize {
    let mut score: usize = 0;
    for window in path.windows(2) {
        let (node_1, node_2) = (window[0], window[1]);
        if (node_1.0, node_1.1) == (node_2.0, node_2.1) {
            score += 1000;
        } else {
            score += 1;
        }
    }
    score
}

fn edge_distance(current: (usize, usize, char), neighbour: (usize, usize, char)) -> usize {
    if neighbour.2 == current.2 {
        return 1;
    }
    return 1000;
}

fn find_neighbours(
    current: (usize, usize, char),
    goal: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
) -> Vec<(usize, usize, char)> {
    let mut neighours: Vec<(usize, usize, char)> = Vec::new();
    if (current.0, current.1) == goal {
        return neighours;
    }
    for ch in "<^>v".chars() {
        if ch == current.2 {
            let next_loc = next_xy(current.0, current.1, ch).unwrap();
            let next_ch = maze.get(&next_loc).unwrap();
            if *next_ch != '#' {
                neighours.push((next_loc.0, next_loc.1, ch));
            }
        } else {
            match (current.2, ch) {
                ('^', 'v') => continue,
                ('>', '<') => continue,
                ('v', '^') => continue,
                ('<', '>') => continue,
                _ => neighours.push((current.0, current.1, ch)),
            }
        }
    }
    neighours
}

fn reconstruct_path(
    came_from: &HashMap<(usize, usize, char), (usize, usize, char)>,
    current: (usize, usize, char),
) -> Vec<(usize, usize, char)> {
    let mut total_path: Vec<(usize, usize, char)> = Vec::new();
    total_path.insert(0, current);
    let mut current = current;
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        total_path.insert(0, current);
    }
    total_path
}

fn min_in_queue(
    queue: &HashSet<(usize, usize, char)>,
    distances: &mut HashMap<(usize, usize, char), usize>,
) -> (usize, usize, char) {
    let mut min_distance = usize::MAX;
    let mut min_node: (usize, usize, char) = (0, 0, '>');
    for node in queue.iter() {
        let distance = *distances.entry(*node).or_insert(usize::MAX);
        if distance < min_distance {
            min_distance = distance;
            min_node = *node;
        }
    }
    min_node
}

fn manhattan_distance(start: (usize, usize), goal: (usize, usize)) -> usize {
    return (goal.0.abs_diff(start.1)) + (goal.1.abs_diff(start.1));
}

fn a_star_search(
    start: (usize, usize, char),
    goal: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
) -> Option<Vec<(usize, usize, char)>> {
    let mut open_set: HashSet<(usize, usize, char)> = HashSet::new();
    open_set.insert(start);

    let mut came_from: HashMap<(usize, usize, char), (usize, usize, char)> = HashMap::new();

    let mut g_score: HashMap<(usize, usize, char), usize> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<(usize, usize, char), usize> = HashMap::new();
    f_score.insert(start, manhattan_distance((start.0, start.1), goal));

    while !open_set.is_empty() {
        let current = min_in_queue(&open_set, &mut f_score);
        if (current.0, current.1) == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        open_set.remove(&current);
        let neighbours = find_neighbours(current, goal, &maze);
        for neighbour in neighbours {
            let tentative_g_score = *g_score
                .get(&current)
                .expect("Should be able to get current gscore")
                + edge_distance(current, neighbour);
            let g_score_neighbour = *g_score.entry(neighbour).or_insert(usize::MAX);
            if tentative_g_score < g_score_neighbour {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(
                    neighbour,
                    tentative_g_score + manhattan_distance((neighbour.0, neighbour.1), goal),
                );
                if !open_set.contains(&neighbour) {
                    open_set.insert(neighbour);
                }
            }
        }
    }
    return None;
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let maze: HashMap<(usize, usize), char> = input_to_grid_map(&input);
    let start_loc = match find_char(&maze, 'S') {
        Some(s) => s,
        None => panic!("Can't find start location"),
    };
    let end_loc = match find_char(&maze, 'E') {
        Some(s) => s,
        None => panic!("Can't find end location"),
    };

    let solution = match a_star_search((start_loc.0, start_loc.1, '>'), end_loc, &maze) {
        Some(s) => s,
        None => panic!("No solution found"),
    };

    score(&solution)
}

fn recursive_optimal_paths(
    current: (usize, usize, char),
    start: (usize, usize, char),
    previous: &HashMap<(usize, usize, char), Vec<(usize, usize, char)>>,
) -> Vec<Vec<(usize, usize, char)>> {
    if current == start {
        return vec![vec![start]];
    }

    let mut all_solutions: Vec<Vec<(usize, usize, char)>> = Vec::new();
    for node in previous
        .get(&current)
        .expect("Should be able to go backwards from here")
    {
        let solutions = recursive_optimal_paths(*node, start, previous);
        for mut solution in solutions {
            solution.insert(0, current);
            all_solutions.push(solution)
        }
    }
    all_solutions
}

fn get_optimal_paths(
    start: (usize, usize, char),
    goal: (usize, usize),
    distances: &HashMap<(usize, usize, char), usize>,
    previous: &HashMap<(usize, usize, char), Vec<(usize, usize, char)>>,
) -> Vec<Vec<(usize, usize, char)>> {
    let (_, min_distance) = distances
        .iter()
        .filter(|(node, _)| (node.0, node.1) == goal)
        .min_by(|(_, d_a), (_, d_b)| d_a.cmp(d_b))
        .unwrap();
    let mut all_solutions: Vec<Vec<(usize, usize, char)>> = Vec::new();
    for (node, _) in distances
        .iter()
        .filter(|(node, d)| (*d == min_distance) & ((node.0, node.1) == goal))
    {
        let solutions = recursive_optimal_paths(*node, start, previous);
        for solution in solutions {
            all_solutions.push(solution)
        }
    }
    all_solutions
}

fn dijktra(
    start: (usize, usize, char),
    goal: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
) -> (
    HashMap<(usize, usize, char), usize>,
    HashMap<(usize, usize, char), Vec<(usize, usize, char)>>,
) {
    let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
    let mut previous: HashMap<(usize, usize, char), Vec<(usize, usize, char)>> = HashMap::new();
    let mut distances: HashMap<(usize, usize, char), usize> = HashMap::new();
    distances.insert(start, 0);

    let mut queue: HashSet<(usize, usize, char)> = HashSet::new();
    queue.insert(start);

    while !queue.is_empty() {
        let current = min_in_queue(&queue, &mut distances);
        queue.remove(&current);
        visited.insert(current);

        let neighbours = find_neighbours(current, goal, maze);
        for neighbour in neighbours {
            if !visited.contains(&neighbour) {
                let alt = *distances.entry(current).or_insert(usize::MAX)
                    + edge_distance(current, neighbour);
                if alt < *distances.entry(neighbour).or_insert(usize::MAX) {
                    let prev = previous.entry(neighbour).or_insert(Vec::new());
                    prev.drain(..);
                    prev.push(current);
                    distances.insert(neighbour, alt);
                } else if alt == *distances.entry(neighbour).or_insert(usize::MAX) {
                    let prev = previous.entry(neighbour).or_insert(Vec::new());
                    prev.push(current);
                }
                queue.insert(neighbour);
            }
        }
    }
    (distances, previous)
}

pub fn part_2(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let maze: HashMap<(usize, usize), char> = input_to_grid_map(&input);
    let start_loc = match find_char(&maze, 'S') {
        Some(s) => s,
        None => panic!("Can't find start location"),
    };
    let end_loc = match find_char(&maze, 'E') {
        Some(s) => s,
        None => panic!("Can't find end location"),
    };
    let start = (start_loc.0, start_loc.1, '>');
    let (distances, previous) = dijktra(start, end_loc, &maze);
    let optimal_paths = get_optimal_paths(start, end_loc, &distances, &previous);

    let mut best_seats: HashSet<(usize, usize)> = HashSet::new();
    for solution in optimal_paths {
        for (x, y, _) in solution {
            best_seats.insert((x, y));
        }
    }
    best_seats.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("data/day16/test.txt");
        assert_eq!(result, 7036);
        let result = part_1("data/day16/test2.txt");
        assert_eq!(result, 11048);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("data/day16/test.txt");
        assert_eq!(result, 45);
        let result = part_2("data/day16/test2.txt");
        assert_eq!(result, 64);
    }
}
