use std::collections::HashMap;
use std::fs;

pub fn blink_stones(times: u16) -> usize {
    let content = fs::read_to_string("./input11.txt").unwrap();
    // println!("{}", content);
    let stones: Vec<_> = content.split(" ").map(|x| String::from(x)).collect();

    let mut sum = 0;
    for stone in stones.iter() {
        println!("stone {}", stone);
        let mut sub_result: HashMap<(String, u16), usize> = HashMap::new();
        sum += blink_stones_times(stone.to_string(), &mut sub_result, times)
    }
    sum
}

fn blink_stones_times(
    stone: String,
    sub_result: &mut HashMap<(String, u16), usize>,
    times: u16,
) -> usize {
    // println!("{}", times);
    if times == 0 {
        return 1;
    }

    let mut sum = 0;

    let sum_mem = sub_result.get(&(stone.to_string(), times));
    if sum_mem.is_some() {
        return *sum_mem.unwrap();
    }
    let result = blink_stone(&stone);

    for stone_result in result.iter() {
        let partial = blink_stones_times(stone_result.to_string(), sub_result, times - 1);

        sum += partial
    }

    sub_result.insert((stone, times), sum);
    sum
}

fn blink_stone(stone: &String) -> Vec<String> {
    let stone_value = stone.parse::<u64>().unwrap();
    if stone.eq("0") {
        return vec![String::from("1")];
    }

    if stone.len() % 2 == 0 {
        let half = stone.len() / 2;
        let left = stone[0..half].parse::<u64>().unwrap().to_string();
        let right = stone[half..stone.len()].parse::<u64>().unwrap().to_string();

        return vec![left, right];
    }

    let new_stone = (2024 * stone_value).to_string();
    vec![new_stone]
}
