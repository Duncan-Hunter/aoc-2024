use std::{fs, str::FromStr};

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
