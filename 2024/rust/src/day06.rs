use crate::common::read_matrix;
use std::collections::{HashMap, HashSet};

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub fn get_num_of_obstacles() -> usize {
    let mut num_obstacles: usize = 0;
    let mut matrix = read_matrix("./input6.txt");

    let (positions, _) = get_positions(&matrix);

    for (pos_x, pos_y) in positions.iter() {
        if matrix[*pos_x][*pos_y].eq("^") {
            continue;
        }
        matrix[*pos_x][*pos_y] = String::from("#");
        let (_, is_stuck) = get_positions(&matrix);
        if is_stuck {
            num_obstacles += 1;
        }

        matrix[*pos_x][*pos_y] = String::from(".")
    }

    num_obstacles
}

pub fn get_positions_len() -> usize {
    let matrix = read_matrix("./input6.txt");
    let (positions, _) = get_positions(&matrix);
    positions.len()
}

pub fn get_positions(matrix: &Vec<Vec<String>>) -> (HashSet<(usize, usize)>, bool) {
    let mut position = find_initial_position(matrix);
    let mut current_direction = Direction::UP;

    let mut visited_position: HashSet<(usize, usize)> = HashSet::new();
    visited_position.insert(position);
    let mut visited_position_direction: HashMap<(usize, usize), String> = HashMap::new();
    visited_position_direction.insert(position, direction_to_string(&current_direction));
    let mut is_stuck = false;

    loop {
        let (pos_x, pos_y) = position;
        match current_direction {
            Direction::UP => {
                let (new_pos_x, out_bound) = pos_x.overflowing_sub(1);
                if out_bound {
                    break;
                }
                if matrix[new_pos_x][pos_y].eq("#") {
                    current_direction = Direction::RIGHT;
                    continue;
                }
                position = (new_pos_x, pos_y);
                visited_position.insert(position);

                let has_been_visited = match visited_position_direction.get(&position) {
                    Some(x) => x.eq(&direction_to_string(&current_direction)),
                    None => false,
                };
                if has_been_visited {
                    is_stuck = true;
                    break;
                }

                visited_position_direction
                    .insert(position, direction_to_string(&current_direction));
            }
            Direction::RIGHT => {
                let new_pos_y = pos_y + 1;
                if new_pos_y >= matrix[pos_x].len() {
                    break;
                }
                if matrix[pos_x][new_pos_y].eq("#") {
                    current_direction = Direction::DOWN;
                    continue;
                }
                position = (pos_x, new_pos_y);
                visited_position.insert(position);

                let has_been_visited = match visited_position_direction.get(&position) {
                    Some(x) => x.eq(&direction_to_string(&current_direction)),
                    None => false,
                };
                if has_been_visited {
                    is_stuck = true;
                    break;
                }
                visited_position_direction
                    .insert(position, direction_to_string(&current_direction));
            }
            Direction::DOWN => {
                let new_pos_x = pos_x + 1;
                if new_pos_x >= matrix.len() {
                    break;
                }
                if matrix[new_pos_x][pos_y].eq("#") {
                    current_direction = Direction::LEFT;
                    continue;
                }
                position = (new_pos_x, pos_y);
                visited_position.insert(position);

                let has_been_visited = match visited_position_direction.get(&position) {
                    Some(x) => x.eq(&direction_to_string(&current_direction)),
                    None => false,
                };
                if has_been_visited {
                    is_stuck = true;
                    break;
                }
                visited_position_direction
                    .insert(position, direction_to_string(&current_direction));
            }
            Direction::LEFT => {
                let (new_pos_y, out_bounds) = pos_y.overflowing_sub(1);
                if out_bounds {
                    break;
                }
                if matrix[pos_x][new_pos_y].eq("#") {
                    current_direction = Direction::UP;
                    continue;
                }
                position = (pos_x, new_pos_y);
                visited_position.insert(position);

                let has_been_visited = match visited_position_direction.get(&position) {
                    Some(x) => x.eq(&direction_to_string(&current_direction)),
                    None => false,
                };
                if has_been_visited {
                    is_stuck = true;
                    break;
                }
                visited_position_direction
                    .insert(position, direction_to_string(&current_direction));
            }
        };
    }

    (visited_position, is_stuck)
}

fn find_initial_position(matrix: &Vec<Vec<String>>) -> (usize, usize) {
    let matrix_len_x = matrix.len();

    for i in 0..matrix_len_x {
        let matrix_len_y = matrix[i].len();
        for j in 0..matrix_len_y {
            if matrix[i][j].eq("^") {
                return (i, j);
            }
        }
    }
    panic!("Missing start point")
}

fn direction_to_string(dir: &Direction) -> String {
    match dir {
        Direction::UP => String::from("UP"),
        Direction::RIGHT => String::from("RIGHT"),
        Direction::DOWN => String::from("DOWN"),
        Direction::LEFT => String::from("LEFT"),
    }
}
