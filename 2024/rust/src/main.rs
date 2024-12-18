mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day12;

fn main() {
    if false {
        let (list1, list2) = day01::read_lists();
        //day 01
        println!("distance: ");
        println!("{}", day01::calculate_distance(&list1, &list2));
        println!("similarity: ");
        println!("{}", day01::calculate_similarity(&list1, &list2));

        //day 02
        let (safe_reports, safe_reports_extended) = day02::check_reports();
        println!("safe reports: {}", safe_reports);
        println!("safe reports extended {} ", safe_reports_extended);

        //day 03
        println!("ops");
        let result = day03::calculate_mul();
        println!("{}", result);

        println!("restricted ops");
        let result_restricted = day03::calculate_mul_restricted();
        println!("{}", result_restricted);

        //day 04
        let sum = day04::find_xmas_all();
        println!("XMAS count: {}", sum);

        let sum_x = day04::find_mas_x();
        println!("X-MAS count: {}", sum_x);

        //day 05
        let sum_valid_orders = day05::sum_valid_orders();
        println!("valid orders sum {}", sum_valid_orders);

        let sum_invalid_orders_corrected = day05::sum_invalid_orders_corrected();
        println!(
            "invalid orders corrected sum {}",
            sum_invalid_orders_corrected
        );

        //day 6
        let visited_positions = day06::get_positions_len();
        println!("visited positions {}", visited_positions);

        let num_of_obstacles = day06::get_num_of_obstacles();
        println!("num_of_obstacles {}", num_of_obstacles);

        //day 7
        println!("{}", day07::read_lists());
        println!("{}", day07::read_lists2());

        //day 8
        day08::find_antinodes();
        day08::find_antinodes_with_resonances();

        // day 10
        let (score, score2) = day10::find_tail_heads();
        println!("score1 {}, score2 {}", score, score2);

        //day 12
        day12::read_map();
    }

    day09::calculate_checksum();
    day09::calculate_checksum2()
}
