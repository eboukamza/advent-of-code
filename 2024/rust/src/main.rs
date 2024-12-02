mod day01;
mod common;
mod day02;

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
}
