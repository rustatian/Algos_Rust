use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("data/day6.txt").unwrap();
    let br = BufReader::new(file);
    let res1 = part_1(br);
    let file = std::fs::File::open("data/day6.txt").unwrap();
    let br2 = BufReader::new(file);
    let res2 = part_2(br2);

    println!("total yes: {}", res1);
    println!("total yes2: {}", res2);
}

fn part_1(br: BufReader<File>) -> usize {
    let mut cur_line = String::new();
    let mut total_yes = 0;
    for l in br.lines() {
        let line = l.unwrap();
        if line == "" {
            let mut line = cur_line.as_bytes().to_vec();
            line.sort_unstable();

            line.dedup_by(|a, b| a == b);

            let new_str = String::from_utf8(line).unwrap();
            total_yes += new_str.len();
            cur_line = String::new();
            continue;
        }
        cur_line = format!("{}{}", cur_line, line);
    }

    total_yes
}

// using binary AND
fn part_2(br: BufReader<File>) -> usize {
    let mut binary_and = None;
    let mut total = 0;
    for l in br.lines() {
        let ll = l.unwrap();
        if ll.is_empty() {
            if binary_and.is_some() {
                let data: HashSet<char> = binary_and.take().unwrap();
                total += data.len();
                binary_and = None;
            }
            continue;
        }

        let line: HashSet<_> = ll.chars().collect();
        if let Some(binary_and) = binary_and.as_mut() {
            *binary_and = &*binary_and & &line;
        } else {
            binary_and = Some(line);
        }
    }

    total
}
