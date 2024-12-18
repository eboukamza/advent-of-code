use std::{fs, usize};

pub fn calculate_checksum() {
    let content = fs::read_to_string("./input9.txt").unwrap();

    let mut disk_files = vec![];
    let mut spaces = vec![];
    let chars = content.chars();
    for (pos, char) in chars.enumerate() {
        let value = usize::try_from(char.to_string().parse::<u32>().unwrap()).unwrap();
        if pos % 2 == 0 {
            let mut file = vec![pos / 2; value];
            disk_files.append(&mut file);
        } else {
            spaces.push(value);
        }
    }

    let mut pos = 0;
    let mut it = 0;
    let mut item = disk_files[0];

    while it < spaces.len() && pos < disk_files.len() - 1 {
        pos += 1;

        if item == disk_files[pos] {
            continue;
        }
        // fill spaces
        let num_spaces_to_fill = spaces[it];
        if pos + num_spaces_to_fill >= disk_files.len() {
            break;
        }
        for _i in 0..num_spaces_to_fill {
            let elem = disk_files.pop().unwrap();
            disk_files.insert(pos, elem);
            pos += 1;
        }
        it += 1;
        item = disk_files[pos];
    }

    let checksum = sum_checksum(&disk_files);
    println!("{:?}", checksum);
}

pub fn calculate_checksum2() {
    let content = fs::read_to_string("./input9.txt").unwrap();

    let mut files = vec![];
    let mut spaces = vec![];
    let chars = content.chars();
    let mut total_size = 0;
    for (pos, char) in chars.enumerate() {
        let size = usize::try_from(char.to_string().parse::<u32>().unwrap()).unwrap();
        let init_pos = total_size;
        if pos % 2 == 0 {
            let id = pos / 2;
            files.push((id, init_pos, size));
        } else {
            spaces.push((init_pos, size));
        }
        total_size += size;
    }
    let mut disk = vec![String::from("."); total_size];

    for (id, init_pos, size) in files.iter() {
        for i in *init_pos..init_pos + size {
            let value = id.to_string();
            disk[i] = value;
        }
    }

    let mut it = files.len() - 1;
    while it > 0 {
        let (_, file_pos, file_size) = files[it];

        //find free space
        let mut space_block_pos = 0;
        let mut found = false;
        while space_block_pos < spaces.len() && !found {
            let (free_space_pos, free_space) = spaces[space_block_pos];
            if file_pos < free_space_pos {
                break;
            }
            if free_space >= file_size {
                let remaining = free_space - file_size;
                if remaining > 0 {
                    spaces[space_block_pos] = (free_space_pos + file_size, remaining);
                } else {
                    spaces.remove(space_block_pos);
                }

                for i in 0..file_size {
                    disk[free_space_pos + i] = disk[file_pos + i].clone();
                    disk[file_pos + i] = String::from(".");
                }
                found = true;
            }

            space_block_pos += 1;
        }
        it -= 1;
    }
    // println!("{:?}", spaces);

    let check_sum = disk
        .iter()
        .enumerate()
        .map(|(pos, item)| {
            if item.eq(".") {
                return 0;
            }
            u64::try_from(pos).unwrap() * item.parse::<u64>().unwrap()
        })
        .reduce(|sum, check| sum + check)
        .unwrap();

    // println!("{:?}", disk);
    println!("{:?}", check_sum);
}

fn sum_checksum(disk_files: &Vec<usize>) -> usize {
    disk_files
        .iter()
        .enumerate()
        .map(|(pos, item)| pos * *item)
        .reduce(|sum, check| sum + check)
        .unwrap()
}
