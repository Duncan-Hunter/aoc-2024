use std::ops::Rem;

use crate::util::read_data_from_file;

fn deconcat(num: usize, de_num: usize) -> Option<usize> {
    if num == de_num {
        return Some(num);
    }
    let num = num.to_string();
    let de_num = de_num.to_string();
    if num.ends_with(&de_num) {
        let result_str = &num[..num.len() - de_num.len()];
        Some(result_str.parse::<usize>().expect("Can't parse"))
    } else {
        None
    }
}

fn check_possible_concat(answer: usize, equation: Vec<usize>) -> bool {
    if equation.len() == 1 {
        if *equation.first().unwrap() == answer {
            return true;
        }
        return false;
    }
    let last_value = *equation.last().unwrap();
    let sub_answer = answer.checked_sub(last_value);
    let sub_possible = match sub_answer {
        Some(a) => {
            let new_equation = equation[..equation.len() - 1].to_vec();
            check_possible_concat(a, new_equation)
        }
        None => false,
    };
    let div_possible = match answer.rem(last_value) {
        0 => {
            let a = answer / last_value;
            let new_equation = equation[..equation.len() - 1].to_vec();
            check_possible_concat(a, new_equation)
        }
        _ => false,
    };
    let concat_possible = match deconcat(answer, last_value) {
        Some(a) => check_possible_concat(a, equation[..equation.len() - 1].to_vec()),
        None => false,
    };
    div_possible | sub_possible | concat_possible
}

fn check_possible(answer: usize, equation: Vec<usize>) -> bool {
    // do this recursively
    if equation.len() == 1 {
        if *equation.first().unwrap() == answer {
            return true;
        }
        return false;
    }
    let last_value = *equation.last().unwrap();
    let sub_answer = answer.checked_sub(last_value);
    let sub_possible = match sub_answer {
        Some(a) => {
            let new_equation = equation[..equation.len() - 1].to_vec();
            check_possible(a, new_equation)
        }
        None => false,
    };
    let div_possible = match answer.rem(last_value) {
        0 => {
            let a = answer / last_value;
            let new_equation = equation[..equation.len() - 1].to_vec();
            check_possible(a, new_equation)
        }
        _ => false,
    };
    let possible = div_possible | sub_possible;
    return possible;
}

fn input_to_equations(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
    for line in input.lines() {
        let (test_value, equation) = line.split_once(':').expect("Can't read line");
        let test_value = test_value
            .trim()
            .parse::<usize>()
            .expect("Can't convert test_value to usize");
        let equation = equation
            .split_ascii_whitespace()
            .map(|x| {
                x.trim()
                    .parse::<usize>()
                    .expect("Can't convert equation to usiz")
            })
            .collect::<Vec<usize>>();
        equations.push((test_value, equation));
    }
    equations
}

pub fn part_1(input: &str) -> usize {
    let input = read_data_from_file(input);
    let equations = input_to_equations(&input);
    let mut total: usize = 0;
    for (answer, equation) in equations {
        let possible = check_possible(answer, equation);
        if possible {
            total += answer;
        }
    }
    total
}

pub fn part_2(input: &str) -> usize {
    let input = read_data_from_file(input);
    let equations = input_to_equations(&input);
    let mut total: usize = 0;
    for (answer, equation) in equations {
        let possible = check_possible_concat(answer, equation);
        if possible {
            total += answer;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_possible() {
        let answer: usize = 190;
        let equation: Vec<usize> = vec![10, 19];
        assert_eq!(check_possible(answer, equation), true);
        let answer: usize = 83;
        let equation: Vec<usize> = vec![17, 5];
        assert_eq!(check_possible(answer, equation), false);
        let answer: usize = 3267;
        let equation: Vec<usize> = vec![81, 40, 27];
        assert_eq!(check_possible(answer, equation), true);
    }

    #[test]
    fn test_input_to_equations() {
        let input = read_data_from_file("data/day7/test.txt");
        let equations = input_to_equations(&input);
        assert_eq!(equations.len(), 9);
        assert_eq!(equations[0].0, 190);
        assert_eq!(equations[8].1[2], 16);
    }

    #[test]
    fn test_deconcat() {
        let num = 615;
        let de_num = 15;
        assert_eq!(deconcat(num, de_num), Some(6));
    }

    #[test]
    fn test_check_possible_concat() {
        let answer: usize = 7290;
        let equation: Vec<usize> = vec![6, 8, 6, 15];
        assert_eq!(check_possible_concat(answer, equation), true);
    }

    #[test]
    fn test_part_1() {
        let answer = part_1("data/day7/test.txt");
        assert_eq!(answer, 3749);
    }

    #[test]
    fn part_2_works() {
        let answer = part_2("data/day7/test.txt");
        assert_eq!(answer, 11387);
    }
}
