use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_matrix(filename: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(filename).unwrap();

    let mut matrix: Vec<Vec<String>> = Vec::new();

    let lines: Vec<_> = contents.split("\n").collect();
    for line in lines.iter() {
        let letters: Vec<_> = line
            .split("")
            .filter(|x| !x.is_empty())
            .map(|s| String::from(s))
            .collect();
        matrix.push(letters);
    }
    matrix
}
