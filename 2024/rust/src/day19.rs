use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub fn count_towel_patterns() {
    let contents = fs::read_to_string("./input19.txt").unwrap();
    let lines: Vec<_> = contents.split("\n").collect();

    let mut patterns: Vec<_> = lines[0].split(", ").collect();

    patterns.sort_by(|a, b| {
        if a.len() > b.len() {
            return Ordering::Greater;
        }
        Ordering::Less
    });

    let mut possibles = vec![];

    for i in 2..lines.len() {
        let design = lines[i];
        let result = is_possible(design, &patterns);
        if result {
            possibles.push(design);
        }
    }

    let mut sum = 0;
    let mut sub_possible: HashMap<String, u64> = HashMap::new();
    println!("num of designs: {}", possibles.len());

    for design in possibles {
        sum += count_possibles(design, &patterns, &mut sub_possible);
    }
    println!("num of combinations {}", sum);
}

fn is_possible(design: &str, patters: &Vec<&str>) -> bool {
    let len = design.len();
    if len == 0 {
        return true;
    }
    for pattern in patters.iter() {
        if design.starts_with(pattern) {
            let rest = &design[pattern.len()..len];
            // println!("{} -> ({}) {}", design, pattern, rest);
            if is_possible(rest, patters) {
                return true;
            }
        }
    }
    false
}

fn count_possibles(
    design: &str,
    patterns: &Vec<&str>,
    sub_possible: &mut HashMap<String, u64>,
) -> u64 {
    let len = design.len();
    if len == 0 {
        return 1;
    }

    let mut count = 0;

    let num_possibles_mem = sub_possible.get(design);
    if num_possibles_mem.is_some() {
        return *num_possibles_mem.unwrap();
    }

    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            let rest = &design[pattern.len()..len];

            let num_possibles = count_possibles(rest, patterns, sub_possible);
            if num_possibles >= 1 {
                count += num_possibles;
                sub_possible.insert(String::from(rest), num_possibles);
            } else {
                sub_possible.insert(String::from(rest), 0);
            }
        }
    }

    count
}
