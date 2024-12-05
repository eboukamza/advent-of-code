use crate::common;
use ordered_vec::OrdVec;

pub fn calculate_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for (index, val1) in list1.iter().enumerate() {
        let val2 = &list2[index];

        sum += (val1 - val2).abs();
    }
    sum
}

pub fn calculate_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut total_similarity: i32 = 0;
    for val1 in list1.iter() {
        let count = list2.iter().filter(|&val2| val1 == val2).count() as i32;

        total_similarity += val1 * count;
    }
    total_similarity
}

pub fn read_lists() -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    if let Ok(lines) = common::read_lines("./input.txt") {
        for line in lines.flatten() {
            let values: Vec<&str> = line.split("   ").collect();
            let val1 = values[0].parse::<i32>().unwrap();
            let val2 = values[1].parse::<i32>().unwrap();
            list1.push_ord_ascending(val1).unwrap();
            list2.push_ord_ascending(val2).unwrap();
        }
    }
    (list1, list2)
}
