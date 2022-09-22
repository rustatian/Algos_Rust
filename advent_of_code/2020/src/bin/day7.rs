use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = include_str!("../../data/day7_test.txt");

    let mut f = File::open("data/day7_test.txt").unwrap();
    let br = BufReader::new(f);

    let mut total = 0;
    for l in br.lines() {
        let line = l.unwrap();
        let line: Vec<_> = line.split("contain").collect();
        if line[1].contains("shiny gold") {
            total += 1;
        }
    }

    println!("total: {}", total);
}
