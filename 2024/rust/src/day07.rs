use crate::common;

pub fn read_lists() -> u64 {
    let mut result_sum = 0;

    if let Ok(lines) = common::read_lines("./input7.txt") {
        for line in lines.flatten() {
            let values = line.split(": ").collect::<Vec<&str>>();
            let result = values[0].parse::<u64>().unwrap();
            let left_values: Vec<_> = values[1]
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let mut left_values_rev: Vec<_> = left_values.iter().rev().collect();
            let first_elem = left_values_rev.pop().unwrap();
            let has_result = find_result(result, *first_elem, &left_values_rev);
            if has_result {
                result_sum += result
            }
        }
    }
    result_sum
}

pub fn read_lists2() -> u64 {
    let mut result_sum = 0;

    if let Ok(lines) = common::read_lines("./input7.txt") {
        for line in lines.flatten() {
            let values = line.split(": ").collect::<Vec<&str>>();
            let result = values[0].parse::<u64>().unwrap();
            let left_values: Vec<_> = values[1]
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let mut left_values_rev: Vec<_> = left_values.iter().rev().collect();
            let first_elem = left_values_rev.pop().unwrap();
            let has_result = find_result_concat(result, *first_elem, &left_values_rev);
            if has_result {
                result_sum += result
            }
        }
    }
    result_sum
}

fn find_result_concat(result: u64, partial: u64, list: &Vec<&u64>) -> bool {
    if partial > result {
        return false;
    }

    let mut new_list = list.clone();
    let last_elem = match new_list.pop() {
        Some(el) => el,
        None => return partial == result,
    };
    let new_partial_1 = partial + last_elem;
    let new_partial_2 = partial * last_elem;

    let mut concat = partial.to_string();
    concat.push_str(&last_elem.to_string());
    let new_partial_3 = concat.parse::<u64>().unwrap();

    find_result_concat(result, new_partial_1, &new_list)
        || find_result_concat(result, new_partial_2, &new_list)
        || find_result_concat(result, new_partial_3, &new_list)
}

fn find_result(result: u64, partial: u64, list: &Vec<&u64>) -> bool {
    if partial > result {
        return false;
    }
    let mut new_list = list.clone();
    let last_elem = match new_list.pop() {
        Some(el) => el,
        None => return partial == result,
    };
    let new_partial_1 = partial + last_elem;
    let new_partial_2 = partial * last_elem;

    find_result(result, new_partial_1, &new_list) || find_result(result, new_partial_2, &new_list)
}
