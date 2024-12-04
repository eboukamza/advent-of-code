use std::fs;

pub fn find_xmas_all() -> i32 {

   let matrix = &read_matrix();

    let mut sum = 0;
    let word = "XMAS";
    let matrix_len = matrix.len();

    for i in 0..matrix_len {
        for j in 0..matrix_len {
            if matrix[i][j].ne("X") {
                continue
            }

            if can_look_up(i, 4) {
                if look_up(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }

            if can_look_down(i, 4, matrix_len) {
                if look_down(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }

            if can_look_right(j, 4, matrix_len) {
                if look_right(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }

            if can_look_left(j, 4) {
                if look_left(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }

            if can_look_up_right(i, j, 4, matrix_len) {
                if look_up_right(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }

            if can_look_up_left(i, j, 4) {
                if look_up_left(i, j, 4, matrix).eq(word) {
                   sum += 1;
                }
            }

            if can_look_down_right(i, j, 4, matrix_len) {
                if look_down_right(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }

            if can_look_down_left(i, j, 4, matrix_len) {
                if look_down_left(i, j, 4, matrix).eq(word) {
                    sum += 1;
                }
            }
        }
    }

    sum
}

pub fn find_mas_x() -> i32 {
    let matrix = &read_matrix();
    let mut sum = 0;

    let matrix_len = matrix.len();

    for i in 0..matrix_len {
        for j in 0..matrix_len {
            let current_letter = &matrix[i][j];
            if current_letter.ne("M") && current_letter.ne("S") {
                continue
            }

            if current_letter.eq("M") {
                if can_look_down_right(i, j, 3, matrix_len) {
                    if look_down_right(i, j, 3, matrix).eq("MAS") {
                       if can_look_down_left(i, j + 2, 3, matrix_len) {
                           let word = look_down_left(i, j +2, 3, matrix);
                           if word.eq("SAM") || word.eq("MAS") {
                               sum += 1;
                           }
                       }
                    }
                }
            }

            if current_letter.eq("S") {
                if can_look_down_right(i, j, 3, matrix_len) {
                    if look_down_right(i, j, 3, matrix).eq("SAM") {
                        if can_look_down_left(i, j + 2, 3, matrix_len) {
                            let word = look_down_left(i, j +2, 3, matrix);
                            if word.eq("SAM") || word.eq("MAS") {
                                sum += 1;
                            }
                        }
                    }
                }
            }

        }
    }

    sum
}

fn read_matrix() -> Vec<Vec<String>> {
    let contents = fs::read_to_string("./input4.txt").unwrap();

    let mut matrix: Vec<Vec<String>> = Vec::new();

    let lines: Vec<_> = contents.split("\n").collect();
    for line in lines.iter() {
        let letters: Vec<_> = line.split("").filter(|x| !x.is_empty()).map(|s| {String::from(s)}).collect();
        matrix.push(letters);
    }
    matrix
}

fn look_up(pos_x: usize, pos_y:usize, word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for i in 0..word_length {
        let x = matrix[pos_x - i][pos_y].as_str();
        word.push_str(x);

    }
    word
}

fn look_down(pos_x: usize, pos_y:usize, word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for i in 0..word_length {
        let x = matrix[pos_x + i][pos_y].as_str();
        word.push_str(x);
    }

    word
}

fn look_left(pos_x: usize, pos_y:usize, word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for j in 0..word_length {
        let x = matrix[pos_x][pos_y - j].as_str();
        word.push_str(x);
    }

    word
}

fn look_right(pos_x: usize, pos_y:usize,word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for j in 0..word_length {
        let x = matrix[pos_x][pos_y + j].as_str();
        word.push_str(x);
    }

    word
}

fn look_up_right(pos_x: usize, pos_y:usize,word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for i in 0..word_length {
        let x = matrix[pos_x - i][pos_y + i].as_str();
        word.push_str(x);
    }

    word
}

fn look_up_left(pos_x: usize, pos_y:usize,word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for i in 0..word_length {
        let x = matrix[pos_x - i][pos_y - i].as_str();
        word.push_str(x);
    }

    word
}

fn look_down_right(pos_x: usize, pos_y:usize,word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for i in 0..word_length {
        let x = matrix[pos_x + i][pos_y + i].as_str();
        word.push_str(x);
    }

    word
}

fn look_down_left(pos_x: usize, pos_y:usize,word_length: usize, matrix: &Vec<Vec<String>>) -> String {
    let mut word = String::from("");
    for i in 0..word_length {
        let x = matrix[pos_x + i][pos_y - i].as_str();
        word.push_str(x);
    }

    word
}

fn can_look_up(pos: usize, word_length: usize) -> bool {
    i32::try_from(pos).unwrap() - i32::try_from(word_length).unwrap() + 1 >= 0
}

fn can_look_left(pos: usize, word_length: usize) -> bool {
    i32::try_from(pos).unwrap() - i32::try_from(word_length).unwrap() + 1 >= 0
}

fn can_look_down(pos: usize, word_length: usize, matrix_length: usize) -> bool{
    i32::try_from(pos).unwrap() + i32::try_from(word_length).unwrap() - 1  < i32::try_from(matrix_length).unwrap()
}

fn can_look_right(pos: usize, word_length: usize, matrix_length: usize) -> bool {
    i32::try_from(pos).unwrap() + i32::try_from(word_length).unwrap() -1 < i32::try_from(matrix_length).unwrap()
}

fn can_look_up_left(pos_x: usize, pos_y:usize, word_length:usize) -> bool {
    can_look_up(pos_x, word_length) && can_look_left(pos_y, word_length)
}

fn can_look_up_right(pos_x: usize, pos_y: usize, word_length: usize, matrix_length: usize) -> bool {
    can_look_up(pos_x, word_length) && can_look_right(pos_y, word_length, matrix_length)
}

fn can_look_down_left(pos_x: usize, pos_y: usize, word_length: usize, matrix_length: usize) -> bool {
    can_look_down(pos_x, word_length, matrix_length) && can_look_left(pos_y, word_length)
}

fn can_look_down_right(pos_x: usize, pos_y: usize, word_length: usize, matrix_length: usize) -> bool {
    can_look_down(pos_x, word_length, matrix_length) && can_look_right(pos_y, word_length, matrix_length)
}