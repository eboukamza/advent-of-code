mod day01;
mod common;
mod day02;
mod day03;
mod day04;

fn main() {
    let (list1, list2) = day01::read_lists();
    //day 01
    println!("distance: ");
    println!("{}", day01::calculate_distance(&list1, &list2));
    println!("similarity: ");
    println!("{}", day01::calculate_similarity(&list1, &list2));

    // day 02
    let (safe_reports, safe_reports_extended) = day02::check_reports();
    println!("safe reports: {}", safe_reports);
    println!("safe reports extended {} ", safe_reports_extended);

    // day 03
    println!("ops");
    let result = day03::calculate_mul();
    println!("{}", result);

    println!("restricted ops");
    let result_restricted = day03::calculate_mul_restricted();
    println!("{}", result_restricted);

    // day 04
    let sum = day04::find_xmas_all();
    println!("XMAS count: {}", sum);

    let sum_x = day04::find_mas_x();
    println!("X-MAS count: {}", sum_x)
}
