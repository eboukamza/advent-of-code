mod day01;

fn main() {
    let (list1, list2) = day01::read_lists();

    println!("distance: ");
    println!("{}", day01::calculate_distance(&list1, &list2));
    println!("similarity: ");
    println!("{}", day01::calculate_similarity(&list1, &list2));
}
