use std::fs;

pub fn read_data_from_file(uri: &str) -> String {
    let contents = fs::read_to_string(uri).expect("Should have been able to read the file");
    contents
}
