use crate::util::read_data_from_file;

#[derive(Debug, Clone)]
struct File {
    free_blocks: usize,
    blocks: Vec<usize>,
}

impl File {
    fn new(id: usize, filled_blocks: usize, free_blocks: usize) -> Self {
        let mut blocks: Vec<usize> = Vec::new();
        for i in 0..filled_blocks {
            blocks.insert(i, id);
        }
        File {
            free_blocks,
            blocks,
        }
    }
}

fn files(file_map: Vec<usize>) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    for (id, filled_blocks) in file_map.iter().step_by(2).enumerate() {
        let free_blocks = match file_map.get(id * 2 + 1) {
            Some(a) => *a,
            None => 0,
        };
        files.insert(id, File::new(id, *filled_blocks, free_blocks));
    }
    files
}

fn file_map(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|x| {
            x.to_string()
                .parse::<usize>()
                .expect("Can't convert to usize")
        })
        .collect::<Vec<usize>>()
}

pub fn part_1(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let fmap = file_map(&input);
    let mut files = files(fmap);

    let mut forward_index: usize = 0;
    let mut backward_index: usize = files.len() - 1;

    while forward_index != backward_index {
        if files[forward_index].free_blocks > 0 {
            if files[backward_index].blocks.len() > 0 {
                let block = files[backward_index].blocks.remove(0);
                files[forward_index].blocks.push(block);
                files[forward_index].free_blocks -= 1;
            } else {
                backward_index -= 1;
            }
        } else {
            forward_index += 1;
        }
    }
    let checksum = files
        .iter()
        .map(|x| &x.blocks)
        .flatten()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum::<usize>();
    checksum
}

pub fn part_2(input_uri: &str) -> usize {
    let input = read_data_from_file(input_uri);
    let fmap = file_map(&input);
    let mut files = files(fmap);

    let mut backward_index = files.len();

    while backward_index > 0 {
        let backwards_blocks = files[backward_index - 1].blocks.clone();
        let num_original_blocks = backwards_blocks
            .iter()
            .filter(|x| **x == backward_index - 1)
            .collect::<Vec<&usize>>()
            .len();
        for forward_index in 0..backward_index - 1 {
            if files[forward_index].free_blocks >= num_original_blocks {
                for (i, v) in backwards_blocks.iter().enumerate() {
                    if *v == backward_index - 1 {
                        files[forward_index].free_blocks -= 1;
                        files[forward_index].blocks.push(*v);
                        files[backward_index - 1].blocks[i] = 0;
                    }
                }
                break;
            }
        }
        backward_index -= 1;
    }

    for file in files.iter_mut() {
        if file.free_blocks > 0 {
            for _ in 0..file.free_blocks {
                file.blocks.push(0);
                file.free_blocks = 0;
            }
        }
    }
    let checksum = files
        .iter()
        .map(|x| &x.blocks)
        .flatten()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum::<usize>();
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_map() {
        let result = file_map("2333133121414131402");
        assert_eq!(result.len(), 19);
        assert_eq!(result[0], 2);
        assert_eq!(result[18], 2);
    }

    #[test]
    fn test_files() {
        let fmap: Vec<usize> = vec![1, 2, 3, 4, 5];
        let result = files(fmap);
        assert_eq!(result.len(), 3);
        assert_eq!(result[2].free_blocks, 0);
    }

    #[test]
    fn test_part_1() {
        let result = part_1("data/day9/test2.txt");
        assert_eq!(result, 60);
        let result = part_1("data/day9/test.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("data/day9/test.txt");
        assert_eq!(result, 2858);
    }
}
