// use day1::{part_1, part_2};
// use day2::{part_1, part_2};
// use day3::{part_1, part_2};
// use day4::{part_1, part_2};
// use day5::{part_1, part_2};
// use day6::{part_1, part_2};
// use day7::{part_1, part_2};
// use day8::{part_1, part_2};
// use day9::{part_1, part_2};
// use day10::{part_1, part_2};
// use day11::{part_1, part_2};
// use day12::{part_1, part_2};
// use day13::{part_1, part_2};
// use day14::{part_1, part_2};
// use day15::{part_1, part_2};
// use day16::{part_1, part_2};
// use day17::{part_1, part_2};
use day18::{part_1, part_2};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod util;

fn main() {
    let part_1_answer = part_1("data/day18/puzzle.txt");
    println!("{part_1_answer}");
    let part_2_answer = part_2("data/day18/puzzle.txt");
    println!("{:?}", part_2_answer);
}
