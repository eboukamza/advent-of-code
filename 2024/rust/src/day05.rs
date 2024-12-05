use std::cmp::Ordering;
use std::fs;

pub fn sum_valid_orders() -> u32 {
    let (rules, orders) = read_input();

    orders
        .iter()
        .filter(|&o| valid_manual(o, &rules))
        .map(|manual| {
            let middle = manual.len() >> 1;
            manual[middle]
        })
        .reduce(|sum, val| sum + val)
        .unwrap()
}

pub fn sum_invalid_orders_corrected() -> u32 {
    let (rules, orders) = read_input();

    orders
        .iter()
        .filter(|&o| !valid_manual(o, &rules))
        .map(|incorrect_manual| {
            let mut corrected = incorrect_manual.iter().copied().collect::<Vec<_>>();
            corrected.sort_by(|a, b| {
                if let Some(_rule) = rules.iter().find(|(i, j)| i == a && j == b) {
                    return Ordering::Less;
                }
                Ordering::Equal
            });
            let middle = corrected.len() >> 1;
            corrected[middle]
        })
        .reduce(|sum, val| sum + val)
        .unwrap()
}

fn read_input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let contents = fs::read_to_string("./input5.txt").unwrap();
    let lines: Vec<_> = contents.split("\n").collect();
    let mut rules: Vec<(u32, u32)> = vec![];

    let mut n = 0;
    while !lines[n].is_empty() {
        let rule: Vec<&str> = lines[n].split('|').collect();
        rules.push((
            rule[0].parse::<u32>().unwrap(),
            rule[1].parse::<u32>().unwrap(),
        ));
        n += 1;
    }
    n += 1;

    let mut orders: Vec<Vec<u32>> = vec![];
    for i in n..lines.len() {
        let order: Vec<_> = lines[i]
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        orders.push(order)
    }

    (rules, orders)
}

fn valid_manual(pages: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    let mut antecesors: Vec<u32> = vec![];
    for item in pages.iter() {
        if !pass_rules(item, &antecesors, &rules) {
            return false;
        }
        antecesors.push(*item)
    }
    true
}

fn pass_rules(x: &u32, ancestors: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for (a, b) in rules.iter() {
        for y in ancestors.iter() {
            if x == a && y == b {
                return false;
            }
        }
    }

    true
}
