use std::usize;

use crate::util::read_data_from_file;
use regex::Regex;

fn result_join(result: &Vec<usize>) -> String {
    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn cdv(
    operand: usize,
    register_a: &usize,
    register_b: &usize,
    register_c: &mut usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    let denominator: usize = 2;
    *register_c =
        *register_a / denominator.pow(combo(operand, register_a, register_b, register_c) as u32);
    *instruction_pointer += 2;
    None
}

fn bdv(
    operand: usize,
    register_a: &usize,
    register_b: &mut usize,
    register_c: &usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    let denominator: usize = 2;
    *register_b =
        *register_a / denominator.pow(combo(operand, register_a, register_b, register_c) as u32);
    *instruction_pointer += 2;
    None
}

fn out(
    operand: usize,
    register_a: &usize,
    register_b: &usize,
    register_c: &usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    *instruction_pointer += 2;
    Some(combo(operand, register_a, register_b, register_c) % 8)
}

fn bxc(
    register_b: &mut usize,
    register_c: &usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    let xor = *register_b ^ *register_c;
    *register_b = xor;
    *instruction_pointer += 2;
    None
}

fn jnz(operand: usize, register_a: &usize, instruction_pointer: &mut usize) -> Option<usize> {
    if *register_a != 0 {
        *instruction_pointer = operand;
    } else {
        *instruction_pointer += 2;
    }
    None
}

fn bst(
    operand: usize,
    register_a: &usize,
    register_b: &mut usize,
    register_c: &usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    *register_b = combo(operand, register_a, register_b, register_c) % 8;
    *instruction_pointer += 2;
    None
}

fn bxl(operand: usize, register_b: &mut usize, instruction_pointer: &mut usize) -> Option<usize> {
    let xor = *register_b ^ operand;
    *register_b = xor;
    *instruction_pointer += 2;
    None
}

fn adv(
    operand: usize,
    register_a: &mut usize,
    register_b: &usize,
    register_c: &usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    let denominator: usize = 2;
    *register_a /= denominator.pow(combo(operand, register_a, register_b, register_c) as u32);
    *instruction_pointer += 2;
    None
}

fn combo(operand: usize, register_a: &usize, register_b: &usize, register_c: &usize) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => *register_a,
        5 => *register_b,
        6 => *register_c,
        _ => panic!("Invalid combo operand"),
    }
}

fn process_opcode(
    opcode: usize,
    operand: usize,
    register_a: &mut usize,
    register_b: &mut usize,
    register_c: &mut usize,
    instruction_pointer: &mut usize,
) -> Option<usize> {
    match opcode {
        0 => adv(
            operand,
            register_a,
            register_b,
            register_c,
            instruction_pointer,
        ),
        1 => bxl(operand, register_b, instruction_pointer),
        2 => bst(
            operand,
            register_a,
            register_b,
            register_c,
            instruction_pointer,
        ),
        3 => {
            jnz(operand, register_a, instruction_pointer);
            None
        }
        4 => bxc(register_b, register_c, instruction_pointer),
        5 => out(
            operand,
            register_a,
            register_b,
            register_c,
            instruction_pointer,
        ),
        6 => bdv(
            operand,
            register_a,
            register_b,
            register_c,
            instruction_pointer,
        ),
        7 => cdv(
            operand,
            register_a,
            register_b,
            register_c,
            instruction_pointer,
        ),
        _ => panic!("Invalid opcode"),
    }
}

fn process_instructions(
    program: &Vec<usize>,
    register_a: &mut usize,
    register_b: &mut usize,
    register_c: &mut usize,
    instruction_pointer: &mut usize,
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    loop {
        match program.get(*instruction_pointer) {
            Some(opcode) => {
                let operand = program.get(*instruction_pointer + 1).unwrap();
                match process_opcode(
                    *opcode,
                    *operand,
                    register_a,
                    register_b,
                    register_c,
                    instruction_pointer,
                ) {
                    Some(out) => result.push(out),
                    None => {}
                }
            }
            None => break,
        }
    }
    result
}

fn process_input(input: &str) -> (Vec<usize>, usize, usize, usize) {
    let register_pattern =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let program_pattern = Regex::new(r"Program: ([\d+,]+)").unwrap();

    let (_, [register_a, register_b, register_c]) = register_pattern
        .captures(input)
        .iter()
        .next()
        .unwrap()
        .extract();
    let (_, [program]) = program_pattern
        .captures(input)
        .iter()
        .next()
        .unwrap()
        .extract();
    let register_a = register_a.parse::<usize>().unwrap();
    let register_b = register_b.parse::<usize>().unwrap();
    let register_c = register_c.parse::<usize>().unwrap();
    let program = program
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (program, register_a, register_b, register_c)
}

pub fn part_1(input_uri: &str) -> String {
    let input = read_data_from_file(input_uri);
    let (program, mut register_a, mut register_b, mut register_c) = process_input(&input);
    let result = process_instructions(
        &program,
        &mut register_a,
        &mut register_b,
        &mut register_c,
        &mut 0,
    );
    result_join(&result)
}

fn find(a: usize, i: usize, program: &Vec<usize>) {
    let mut input_a = a;
    let result = process_instructions(&program, &mut input_a, &mut 0, &mut 0, &mut 0);
    if result == *program {
        println!("{a}");
    } else if (program.ends_with(&result)) | (i == 0) {
        for n in 0..8usize {
            find(8 * a + n, i + 1, program);
        }
    }
}

pub fn part_2(input_uri: &str) {
    let input = read_data_from_file(input_uri);
    let (program, _, _, _) = process_input(&input);

    find(0, 0, &program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() {
        let mut register_a = 4;
        let mut register_b = 0;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        let _ = adv(
            2,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(register_a, 1);
        assert_eq!(instruction_pointer, 2);
        let mut register_a = 5;
        let mut register_b = 1;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        let _ = adv(
            5,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(register_a, 2);
        assert_eq!(instruction_pointer, 2);
    }

    #[test]
    fn test_process_instructions() {
        let mut register_a = 10;
        let mut register_b = 0;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        let instructions = vec![5, 0, 5, 1, 5, 4];
        let result = process_instructions(
            &instructions,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(result, vec![0, 1, 2]);

        let mut register_a = 2024;
        let mut register_b = 0;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        let instructions = vec![0, 1, 5, 4, 3, 0];
        let result = process_instructions(
            &instructions,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(result, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(register_a, 0);

        let mut register_a = 729;
        let mut register_b = 0;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        let instructions = vec![0, 1, 5, 4, 3, 0];
        let result = process_instructions(
            &instructions,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(result, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);

        let mut register_a = 117440;
        let mut register_b = 0;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        let instructions = vec![0, 3, 5, 4, 3, 0];
        let result = process_instructions(
            &instructions,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(result, vec![0, 3, 5, 4, 3, 0]);
    }

    #[test]
    fn test_process_opcode() {
        let mut register_a = 0;
        let mut register_b = 0;
        let mut register_c = 9;
        let mut instruction_pointer = 0;
        process_opcode(
            2,
            6,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(register_b, 1);
        assert_eq!(instruction_pointer, 2);

        let mut register_a = 0;
        let mut register_b = 29;
        let mut register_c = 0;
        let mut instruction_pointer = 0;
        process_opcode(
            1,
            7,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(register_b, 26);
        assert_eq!(instruction_pointer, 2);

        let mut register_a = 0;
        let mut register_b = 2024;
        let mut register_c = 43690;
        let mut instruction_pointer = 0;
        process_opcode(
            4,
            0,
            &mut register_a,
            &mut register_b,
            &mut register_c,
            &mut instruction_pointer,
        );
        assert_eq!(register_b, 44354);
        assert_eq!(instruction_pointer, 2);
    }
    #[test]
    fn test_part_1() {
        let result = part_1("data/day17/test.txt");
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
