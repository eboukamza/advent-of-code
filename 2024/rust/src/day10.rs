use crate::common::read_matrix;
use std::collections::HashSet;

pub fn find_tail_heads() -> (usize, u32) {
    let matrix = &read_matrix("./input10.txt");
    let mut score = 0;
    let mut score2 = 0;
    for (i, _line) in matrix.iter().enumerate() {
        for (j, cell) in matrix[i].iter().enumerate() {
            let value = cell.parse::<u32>().unwrap();
            if value == 0 {
                let mut positions = HashSet::new();
                score2 += num_of_tail_heads((i, j), &matrix, &mut positions);
                score += positions.len();
            }
        }
    }
    (score, score2)
}

fn num_of_tail_heads(
    pos: (usize, usize),
    matrix: &Vec<Vec<String>>,
    positions: &mut HashSet<(usize, usize)>,
) -> u32 {
    let (i, j) = pos;
    let value = matrix[i][j].parse::<u32>().unwrap();
    if value == 9 {
        positions.insert((i, j));
        return 1;
    }
    // for each direction look value and if is correct find a tail head
    // is not out of bounds
    let mut sum = 0;
    // up
    let (new_i_up, out_bound_x) = i.overflowing_sub(1);
    if !out_bound_x {
        let up_value: u32 = matrix[new_i_up][j].parse::<u32>().unwrap();
        if up_value == value + 1 {
            sum += num_of_tail_heads((new_i_up, j), matrix, positions);
        }
    }
    // down
    let new_i_down = i + 1;
    if new_i_down < matrix.len() {
        let down_value: u32 = matrix[new_i_down][j].parse::<u32>().unwrap();
        if down_value == value + 1 {
            sum += num_of_tail_heads((new_i_down, j), matrix, positions)
        }
    }

    //left
    let (new_j_left, out_bound_y) = j.overflowing_sub(1);
    if !out_bound_y {
        let down_value: u32 = matrix[i][new_j_left].parse::<u32>().unwrap();
        if down_value == value + 1 {
            sum += num_of_tail_heads((i, new_j_left), matrix, positions);
        }
    }

    //right
    let new_j_right = j + 1;
    if new_j_right < matrix[i].len() {
        let right_value: u32 = matrix[i][new_j_right].parse::<u32>().unwrap();
        if right_value == value + 1 {
            sum += num_of_tail_heads((i, new_j_right), matrix, positions);
        }
    }

    sum
}
