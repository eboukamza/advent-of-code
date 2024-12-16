use crate::common::read_matrix;
use std::collections::{HashMap, HashSet};

pub fn read_map() {
    let matrix = &read_matrix("./input12.txt");

    let mut crop_map: HashMap<String, Vec<Vec<((usize, usize), HashSet<String>)>>> = HashMap::new();

    let mut visited_matrix: Vec<Vec<bool>> = vec![];
    visited_matrix.resize_with(matrix.len(), || {
        let mut line = vec![];
        line.resize_with(matrix[0].len(), || false);
        line
    });

    let mut price_matrix: Vec<Vec<HashSet<String>>> = vec![];
    price_matrix.resize_with(matrix.len(), || {
        let mut line: Vec<HashSet<String>> = vec![];
        line.resize_with(matrix[0].len(), || HashSet::new());
        line
    });

    for (i, _line) in matrix.iter().enumerate() {
        for (j, value) in matrix[i].iter().enumerate() {
            if visited_matrix[i][j] {
                continue;
            }

            let mut region: Vec<((usize, usize), HashSet<String>)> = vec![];
            find_region(
                (i, j),
                matrix,
                &mut visited_matrix,
                &mut price_matrix,
                &mut region,
            );

            if crop_map.get(value).is_none() {
                crop_map.insert(value.to_string(), vec![]);
            }
            let garden_plot = crop_map.get_mut(value).unwrap();
            garden_plot.push(region);
        }
    }

    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    for (_name, regions) in crop_map.into_iter() {
        for region in regions.iter() {
            let area = region.len();
            let perimeter = region
                .iter()
                .map(|(_position, score)| u32::try_from(score.len()).unwrap())
                .reduce(|sum: u32, val| sum + val)
                .unwrap();
            sum += u32::try_from(area).unwrap() * perimeter;

            let sides = region
                .iter()
                .map(|(position, fences)| {
                    let (i, j) = *position;
                    let mut score = u32::try_from(fences.len()).unwrap();

                    if fences.get("U").is_some()
                        && j > 0
                        && matrix[i][j].eq(&matrix[i][j - 1])
                        && price_matrix[i][j - 1].contains("U")
                    {
                        score -= 1;
                    }

                    if fences.get("D").is_some()
                        && j > 0
                        && matrix[i][j].eq(&matrix[i][j - 1])
                        && price_matrix[i][j - 1].contains("D")
                    {
                        score -= 1;
                    }

                    if fences.get("L").is_some()
                        && i > 0
                        && matrix[i][j].eq(&matrix[i - 1][j])
                        && price_matrix[i - 1][j].contains("L")
                    {
                        score -= 1;
                    }

                    if fences.get("R").is_some()
                        && i > 0
                        && matrix[i][j].eq(&matrix[i - 1][j])
                        && price_matrix[i - 1][j].contains("R")
                    {
                        score -= 1;
                    }

                    score
                })
                .reduce(|sum: u32, val| sum + val)
                .unwrap();

            sum2 += u32::try_from(area).unwrap() * sides;
        }
    }
    println!("total price of fencing {}", sum);
    println!("total price of fencing with discount {}", sum2);
}

fn find_region(
    position: (usize, usize),
    matrix: &Vec<Vec<String>>,
    visited_matrix: &mut Vec<Vec<bool>>,
    price_matrix: &mut Vec<Vec<HashSet<String>>>,
    new_region: &mut Vec<((usize, usize), HashSet<String>)>,
) {
    let (i, j) = position;
    let value = &matrix[i][j];
    visited_matrix[i][j] = true;

    let cell_price = calculate_cell_price(value, (i, j), matrix);
    price_matrix[i][j] = cell_price.clone();

    new_region.push(((i, j), cell_price.clone()));

    // up
    let (new_i_up, out_bound_x) = i.overflowing_sub(1);
    if !out_bound_x && !visited_matrix[new_i_up][j] && matrix[new_i_up][j].eq(value) {
        find_region(
            (new_i_up, j),
            matrix,
            visited_matrix,
            price_matrix,
            new_region,
        );
    }
    //down
    let new_i_down = i + 1;
    if new_i_down < matrix.len()
        && !visited_matrix[new_i_down][j]
        && matrix[new_i_down][j].eq(value)
    {
        find_region(
            (new_i_down, j),
            matrix,
            visited_matrix,
            price_matrix,
            new_region,
        );
    }

    //left
    let (new_j_left, out_bound_y) = j.overflowing_sub(1);
    if !out_bound_y && !visited_matrix[i][new_j_left] && matrix[i][new_j_left].eq(value) {
        find_region(
            (i, new_j_left),
            matrix,
            visited_matrix,
            price_matrix,
            new_region,
        );
    }

    //right
    let new_j_right = j + 1;
    if new_j_right < matrix[i].len()
        && !visited_matrix[i][new_j_right]
        && matrix[i][new_j_right].eq(value)
    {
        find_region(
            (i, new_j_right),
            matrix,
            visited_matrix,
            price_matrix,
            new_region,
        );
    }
}

fn calculate_cell_price(
    value: &String,
    position: (usize, usize),
    matrix: &Vec<Vec<String>>,
) -> HashSet<String> {
    let mut result = HashSet::new();
    let (i, j) = position;

    //up
    if i == 0 || matrix[i - 1][j].ne(value) {
        result.insert(String::from("U"));
    }

    //left
    if j == 0 || matrix[i][j - 1].ne(value) {
        result.insert(String::from("L"));
    }

    //down
    if matrix.get(i + 1).is_none() || matrix[i + 1][j].ne(value) {
        result.insert(String::from("D"));
    }

    //right
    if matrix[i].get(j + 1).is_none() || matrix[i][j + 1].ne(value) {
        result.insert(String::from("R"));
    }

    result
}
