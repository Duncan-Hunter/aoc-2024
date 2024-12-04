use crate::util::read_data_from_file;

fn get_char(array: &Vec<Vec<char>>, row: usize, col: usize) -> Option<char> {
    let c = *array.get(row)?.get(col)?;
    Some(c)
}

fn check_xmas(array: &Vec<Vec<char>>, row: usize, col: usize) -> Option<usize> {
    let direction: Vec<isize> = vec![-1, 0, 1];
    let mut xmas_count: usize = 0;
    for row_direction in direction.iter() {
        for col_direction in direction.iter() {
            if (*row_direction == 0) & (*col_direction == 0) {
                continue;
            }
            let m_row = match row.checked_add_signed(row_direction * 1) {
                Some(r) => r,
                None => continue,
            };
            let m_col = match col.checked_add_signed(col_direction * 1) {
                Some(c) => c,
                None => continue,
            };
            match get_char(array, m_row, m_col) {
                Some(m) => {
                    if m != 'M' {
                        continue;
                    }
                }
                None => continue,
            }
            let a_row = match row.checked_add_signed(row_direction * 2) {
                Some(r) => r,
                None => continue,
            };
            let a_col = match col.checked_add_signed(col_direction * 2) {
                Some(c) => c,
                None => continue,
            };
            match get_char(array, a_row, a_col) {
                Some(a) => {
                    if a != 'A' {
                        continue;
                    }
                }
                None => continue,
            }
            let s_row = match row.checked_add_signed(row_direction * 3) {
                Some(r) => r,
                None => continue,
            };
            let s_col = match col.checked_add_signed(col_direction * 3) {
                Some(c) => c,
                None => continue,
            };
            match get_char(array, s_row, s_col) {
                Some(s) => {
                    if s != 'S' {
                        continue;
                    } else {
                        xmas_count += 1
                    }
                }
                None => continue,
            }
        }
    }
    Some(xmas_count)
}

fn check_xs(array: &Vec<Vec<char>>, xs: &Vec<(usize, usize)>) -> usize {
    let mut total_count: usize = 0;
    for (row, col) in xs.iter() {
        match check_xmas(array, *row, *col) {
            Some(total) => total_count += total,
            None => {}
        }
    }
    total_count
}

fn find_xs(array: &Vec<Vec<char>>, c: char) -> Vec<(usize, usize)> {
    let mut xs: Vec<(usize, usize)> = Vec::new();
    for (row, line) in array.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == c {
                xs.push((row, col));
            }
        }
    }
    xs
}

fn create_array(input: &str) -> Vec<Vec<char>> {
    let lines = input.split_ascii_whitespace().collect::<Vec<&str>>();
    let chars = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    chars
}

pub fn part_1() -> () {
    let input = read_data_from_file("data/day4/puzzle.txt");
    let array = create_array(&input);
    let xs = find_xs(&array, 'X');
    let total = check_xs(&array, &xs);
    println!("{total}");
}

fn check_x_mas(array: &Vec<Vec<char>>, row: usize, col: usize) -> Option<usize> {
    let top_left = get_char(
        array,
        row.checked_add_signed(-1)?,
        col.checked_add_signed(-1)?,
    )?;
    let top_right = get_char(array, row.checked_add_signed(-1)?, col + 1)?;
    let bottom_left = get_char(array, row + 1, col.checked_add_signed(-1)?)?;
    let bottom_right = get_char(array, row + 1, col + 1)?;

    if (top_left == 'M') & (top_right == 'M') & (bottom_right == 'S') & (bottom_left == 'S') {
        return Some(1);
    } else if (top_left == 'S') & (top_right == 'M') & (bottom_right == 'M') & (bottom_left == 'S')
    {
        return Some(1);
    } else if (top_left == 'S') & (top_right == 'S') & (bottom_right == 'M') & (bottom_left == 'M')
    {
        return Some(1);
    } else if (top_left == 'M') & (top_right == 'S') & (bottom_right == 'S') & (bottom_left == 'M')
    {
        return Some(1);
    }
    Some(0)
}

fn count_x_mas(array: &Vec<Vec<char>>, middle_as: &Vec<(usize, usize)>) -> usize {
    let mut count: usize = 0;
    for (row, col) in middle_as.iter() {
        match check_x_mas(array, *row, *col) {
            Some(c) => count += c,
            None => {}
        }
    }
    count
}

pub fn part_2() -> () {
    let input = read_data_from_file("data/day4/puzzle.txt");
    let array: Vec<Vec<char>> = create_array(&input);
    let middle_as = find_xs(&array, 'A');
    let total = count_x_mas(&array, &middle_as);
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // assert_eq!
        let input = read_data_from_file("data/day4/test.txt");
        let array = create_array(&input);
        let xs = find_xs(&array, 'X');
        let total = check_xs(&array, &xs);
        assert_eq!(total, 18);
    }

    #[test]
    fn part_2_works() {
        let input = read_data_from_file("data/day4/test.txt");
        let array: Vec<Vec<char>> = create_array(&input);
        let middle_as = find_xs(&array, 'A');
        let total = count_x_mas(&array, &middle_as);
        dbg!(total);
        assert_eq!(total, 9);
    }
}
