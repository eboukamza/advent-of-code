use crate::common;

pub fn check_reports() -> (u32, u32) {
    let mut count_safe = 0;
    let mut count_safe_extended = 0;
    if let Ok(lines) = common::read_lines("./input2.txt") {
        for report in lines.flatten() {
            let levels: Vec<_> = report
                .split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect();

            let is_safe = is_safe(&levels);
            if is_safe {
                count_safe += 1;
                count_safe_extended += 1;
            } else if is_safe_extended(&levels) {
                count_safe_extended += 1;
            }
        }
    }
    (count_safe, count_safe_extended)
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut last_diff = 0; // -1 decreasing, 1 increasing, 0 neutral
    for (index, level) in levels.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let diff = level - levels[index - 1];

        if diff == 0 || diff.abs() > 3 || diff > 0 && last_diff < 0 || diff < 0 && last_diff > 0 {
            return false;
        }

        last_diff = diff;
    }
    true
}

fn is_safe_extended(levels: &Vec<i32>) -> bool {
    if is_safe(levels) {
        return true;
    }
    for (index, _level) in levels.iter().enumerate() {
        let mut sub_levels = levels.clone();
        sub_levels.remove(index);
        if is_safe(&sub_levels) {
            return true;
        }
    }
    false
}
