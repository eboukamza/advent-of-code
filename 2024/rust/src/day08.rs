use crate::common::read_matrix;
use std::collections::HashMap;

pub fn find_antinodes() {
    let matrix = read_matrix("./input8.txt");

    let mut antennas: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes_matrix: Vec<Vec<bool>> = vec![];
    antinodes_matrix.resize_with(matrix.len(), || {
        let mut line = vec![];
        line.resize_with(matrix[0].len(), || false);
        line
    });

    for (i, line) in matrix.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {
            if value.eq(".") {
                continue;
            }
            if antennas.get(value).is_none() {
                antennas.insert(value.to_string(), vec![]);
            }
            antennas.get_mut(value).unwrap().push((i, j));
        }
    }

    for (_antenna, positions) in antennas.iter() {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let antinodes = get_antinodes_coord(positions[i], positions[j], &matrix, 1);
                for (x, y) in antinodes.iter() {
                    antinodes_matrix[*x][*y] = true;
                }
            }
        }
    }

    let mut antennas_sum = 0;
    for line in antinodes_matrix.iter() {
        for value in line.iter() {
            if *value {
                antennas_sum += 1
            }
        }
    }
    println!("{}", antennas_sum)
}

pub fn find_antinodes_with_resonances() {
    let matrix = read_matrix("./input8.txt");

    let mut antennas: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes_matrix: Vec<Vec<bool>> = vec![];
    antinodes_matrix.resize_with(matrix.len(), || {
        let mut line = vec![];
        line.resize_with(matrix[0].len(), || false);
        line
    });

    for (i, line) in matrix.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {
            if value.eq(".") {
                continue;
            }
            if antennas.get(value).is_none() {
                antennas.insert(value.to_string(), vec![]);
            }
            antennas.get_mut(value).unwrap().push((i, j));
        }
    }

    for (_antenna, positions) in antennas.iter() {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let mut it = 1;
                let mut antinodes = vec![positions[i], positions[j]];
                loop {
                    let mut antinodes_coord =
                        get_antinodes_coord(positions[i], positions[j], &matrix, it);
                    if antinodes_coord.is_empty() {
                        break;
                    }

                    antinodes.append(&mut antinodes_coord);
                    it += 1;
                }
                for (x, y) in antinodes.iter() {
                    antinodes_matrix[*x][*y] = true;
                }
            }
        }
    }

    let mut antennas_sum = 0;
    for line in antinodes_matrix.iter() {
        for value in line.iter() {
            if *value {
                antennas_sum += 1
            }
        }
    }
    println!("{}", antennas_sum)
}

fn get_antinodes_coord(
    position_a: (usize, usize),
    position_b: (usize, usize),
    matrix: &Vec<Vec<String>>,
    it: i32,
) -> Vec<(usize, usize)> {
    let (x1, y1) = position_a;
    let (x2, y2) = position_b;

    let x_delta = i32::try_from(x2).unwrap() - i32::try_from(x1).unwrap();
    let y_delta = i32::try_from(y2).unwrap() - i32::try_from(y1).unwrap();

    let a1_x = i32::try_from(x1).unwrap() - x_delta * it;
    let a1_y = i32::try_from(y1).unwrap() - y_delta * it;

    let mut result = vec![];
    if a1_x >= 0 && a1_y >= 0 {
        let (a1_x, a1_y) = (
            usize::try_from(a1_x).unwrap(),
            usize::try_from(a1_y).unwrap(),
        );
        if a1_x < matrix.len() && a1_y < matrix[a1_x].len() {
            result.push((a1_x, a1_y));
        }
    }

    let a2_x = i32::try_from(x2).unwrap() + x_delta * it;
    let a2_y = i32::try_from(y2).unwrap() + y_delta * it;

    if a2_x >= 0 && a2_y >= 0 {
        let (a2_x, a2_y) = (
            usize::try_from(a2_x).unwrap(),
            usize::try_from(a2_y).unwrap(),
        );
        if a2_x < matrix.len() && a2_y < matrix[a2_x].len() {
            result.push((a2_x, a2_y))
        }
    }

    result
}
