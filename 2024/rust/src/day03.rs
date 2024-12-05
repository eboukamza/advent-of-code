use regex::Regex;
use std::fs;

pub fn calculate_mul() -> i32 {
    let contents = fs::read_to_string("./input3.txt").unwrap();
    parse_and_execute(&contents)
}

pub fn calculate_mul_restricted() -> i32 {
    let content = fs::read_to_string("./input3.txt").unwrap();
    let restricted_content = remove_dont_ops(&content);
    parse_and_execute(&restricted_content)
}

fn remove_dont_ops(code: &str) -> String {
    let one_line = code.replace("\n", "");
    let dont_regex = Regex::new(r"don't\(\).*?(?:do\(\)|$)").unwrap();
    dont_regex.replace_all(&one_line, "").to_string()
}

fn parse_and_execute(code: &str) -> i32 {
    let ops = scan_for_mul(code);
    sum_mul_ops(&ops)
}

fn scan_for_mul(code: &str) -> Vec<(i32, i32)> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut ops: Vec<(i32, i32)> = vec![];
    for (_, [op1, op2]) in mul_regex.captures_iter(code).map(|caps| caps.extract()) {
        ops.push((op1.parse::<i32>().unwrap(), op2.parse::<i32>().unwrap()));
    }

    ops
}

fn sum_mul_ops(ops: &Vec<(i32, i32)>) -> i32 {
    ops.iter()
        .map(|(op1, op2)| op1 * op2)
        .reduce(|sum, val| sum + val)
        .unwrap()
}
