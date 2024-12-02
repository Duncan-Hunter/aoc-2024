use crate::util::read_data_from_file;
use std::iter::zip;

fn level_differ(report: &Vec<isize>) -> bool {
    for (i, j) in zip(report, report[1..].iter()) {
        if (i.abs_diff(*j) <= 3) & (i.abs_diff(*j) >= 1) {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn monotonic(report: &Vec<isize>, increasing: bool) -> bool {
    let mut mult = -1;
    if increasing {
        mult = 1;
    }
    for (i, j) in zip(report, report[1..].iter()) {
        if mult * *j - mult * *i > 0 {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn safe(report: &Vec<isize>) -> isize {
    if (monotonic(report, true) | monotonic(report, false)) & level_differ(report) {
        return 1;
    }
    0
}

pub fn part_1() -> () {
    let input_data = read_data_from_file("data/day2/puzzle1.txt");

    let reports: Vec<Vec<isize>> = input_data
        .split("\n")
        .into_iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .into_iter()
                .map(|z| z.trim().parse::<isize>().expect("Can't convert"))
                .collect::<Vec<isize>>()
        })
        .collect();
    let safety = reports.iter().map(|x| safe(x)).collect::<Vec<isize>>();
    let safe_count = safety.iter().sum::<isize>();
    println!("{safe_count}");
}

fn safe_level_removed(report: &Vec<isize>) -> isize {
    let n_levels = report.len();
    for i in 0..n_levels {
        let report_removed = report
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, x)| *x)
            .collect::<Vec<isize>>();
        if safe(&report_removed) == 1 {
            return 1;
        }
    }
    0
}

pub fn part_2() -> () {
    let input_data = read_data_from_file("data/day2/puzzle1.txt");

    let reports: Vec<Vec<isize>> = input_data
        .split("\n")
        .into_iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .into_iter()
                .map(|z| z.trim().parse::<isize>().expect("Can't convert"))
                .collect::<Vec<isize>>()
        })
        .collect();
    let safety = reports
        .iter()
        .map(|x| safe_level_removed(x))
        .collect::<Vec<isize>>();
    let safe_count = safety.iter().sum::<isize>();
    println!("{safe_count}");
}
