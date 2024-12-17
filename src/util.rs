use std::{collections::HashMap, fs, str::FromStr};

pub fn read_data_from_file(uri: &str) -> String {
    let contents = fs::read_to_string(uri).expect("Should have been able to read the file");
    contents
}

pub fn input_to_grid<T: FromStr>(input: &str) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch.to_string().parse::<T>() {
                    Ok(c) => c,
                    Err(_) => panic!("Can't convert to type"),
                })
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

pub fn input_to_grid_map(input: &str) -> HashMap<(usize, usize), char> {
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch);
        }
    }
    grid
}

pub fn display_grid_map(grid: &HashMap<(usize, usize), char>) {
    let mut vec_grid: Vec<Vec<char>> = Vec::new();
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    for i in 0..*max_x + 1 {
        vec_grid.push(Vec::new());
        for j in 0..*max_y + 1 {
            let ch = grid.get(&(i, j)).unwrap();
            vec_grid[i].push(*ch);
        }
    }
    for line in vec_grid {
        let s = line.iter().collect::<String>();
        println!("{s}");
    }
}

pub fn find_char(grid: &HashMap<(usize, usize), char>, ch: char) -> Option<(usize, usize)> {
    for ((i, j), c) in grid {
        if *c == ch {
            return Some((*i, *j));
        }
    }
    None
}

pub fn next_xy(x: usize, y: usize, direction: char) -> Option<(usize, usize)> {
    let direction: (isize, isize) = match direction {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => return None,
    };
    let next_x = x.checked_add_signed(direction.0)?;
    let next_y = y.checked_add_signed(direction.1)?;
    Some((next_x, next_y))
}
