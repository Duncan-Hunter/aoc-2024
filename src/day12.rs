use std::collections::{HashMap, HashSet};

use crate::util::{input_to_grid, read_data_from_file};

fn recursive_region_search(
    row: usize,
    col: usize,
    plant_type: char,
    region_number: usize,
    regions: &mut HashMap<char, HashMap<usize, (HashSet<(usize, usize)>, usize)>>,
    unvisited: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    unvisited.remove(&(row, col));

    let mut bordering: usize = 0;
    for (plus_row, plus_col) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
        let next_row = match row.checked_add_signed(*plus_row) {
            Some(a) => {
                if a < grid.len() {
                    a
                } else {
                    bordering += 1;
                    continue;
                }
            }
            None => {
                bordering += 1;
                continue;
            }
        };
        let next_col = match col.checked_add_signed(*plus_col) {
            Some(a) => {
                if a < grid[0].len() {
                    a
                } else {
                    bordering += 1;
                    continue;
                }
            }
            None => {
                bordering += 1;
                continue;
            }
        };
        if unvisited.contains(&(next_row, next_col)) & (grid[next_row][next_col] == plant_type) {
            recursive_region_search(
                next_row,
                next_col,
                plant_type,
                region_number,
                regions,
                unvisited,
                grid,
            );
        }
        if grid[next_row][next_col] != plant_type {
            bordering += 1;
        }
    }

    let region = regions.entry(plant_type).or_default();
    let (garden_plots, perimeter) = region.entry(region_number).or_default();
    garden_plots.insert((row, col));
    *perimeter += bordering;
}

fn find_regions(
    grid: &Vec<Vec<char>>,
) -> HashMap<char, HashMap<usize, (HashSet<(usize, usize)>, usize)>> {
    let mut regions: HashMap<char, HashMap<usize, (HashSet<(usize, usize)>, usize)>> =
        HashMap::new();
    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            unvisited.insert((i, j));
        }
    }
    while !unvisited.is_empty() {
        let (row, col) = unvisited.iter().next().unwrap();
        let plant_type = grid[*row][*col];
        let region_number = match regions.get(&plant_type) {
            Some(regions) => match regions.keys().max() {
                Some(v) => v + 1,
                None => 0,
            },
            None => 0,
        };
        recursive_region_search(
            *row,
            *col,
            plant_type,
            region_number,
            &mut regions,
            &mut unvisited,
            grid,
        );
    }
    regions
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let grid = input_to_grid::<char>(&input);
    let regions: HashMap<char, HashMap<usize, (HashSet<(usize, usize)>, usize)>> =
        find_regions(&grid);
    let mut total: usize = 0;
    for (_, r) in regions.iter() {
        for (_, (garden_plots, perimeter)) in r.iter() {
            total += garden_plots.len() * perimeter;
        }
    }
    total
}

pub fn part_2(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let grid = input_to_grid::<char>(&input);
    let regions: HashMap<char, HashMap<usize, (HashSet<(usize, usize)>, usize)>> =
        find_regions(&grid);

    let mut corner_count: HashMap<(char, usize), usize> = HashMap::new();
    let mut new_grid: Vec<Vec<(char, usize)>> = Vec::new();
    for i in 0..grid.len() + 2 {
        new_grid.push(Vec::new());
        for _ in 0..grid[0].len() + 2 {
            new_grid[i].push(('.', 0));
        }
    }
    for (plant_type, region) in regions.iter() {
        for (region_number, (garden_plots, _)) in region {
            for (i, j) in garden_plots {
                new_grid[i + 1][j + 1] = (*plant_type, *region_number);
            }
        }
    }
    for i in 0..new_grid.len() - 1 {
        for j in 0..new_grid[0].len() - 1 {
            let window: Vec<(char, usize)> = vec![
                new_grid[i][j],
                new_grid[i][j + 1],
                new_grid[i + 1][j],
                new_grid[i + 1][j + 1],
            ];
            let unique_types = window.iter().collect::<HashSet<&(char, usize)>>();
            for t in unique_types.iter() {
                let count = window.iter().filter(|x| x == t).count();
                if count == 1 {
                    *corner_count.entry(**t).or_insert(0) += 1;
                } else if count == 3 {
                    *corner_count.entry(**t).or_insert(0) += 1;
                } else if count == 2 {
                    if (window[0] == window[3]) | (window[1] == window[2]) {
                        *corner_count.entry(**t).or_insert(0) += 2;
                    }
                }
            }
        }
    }
    corner_count.remove(&('.', 0));
    let mut total: usize = 0;
    for ((plant_type, region_number), fence_count) in corner_count {
        let region_area = regions
            .get(&plant_type)
            .unwrap()
            .get(&region_number)
            .unwrap()
            .0
            .len();
        total += region_area * fence_count;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("data/day12/test.txt");
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("data/day12/test.txt");
        assert_eq!(result, 1206);
        let result = part_2("data/day12/test2.txt");
        assert_eq!(result, 236);
        let result = part_2("data/day12/test3.txt");
        assert_eq!(result, 368);
    }
}
