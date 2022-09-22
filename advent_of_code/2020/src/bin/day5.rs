use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("data/day5.txt").unwrap();

    let reader = BufReader::new(f);

    let mut parser = Parser::new();
    let mut max_id = 0;
    let mut v = vec![];

    for l in reader.lines() {
        let line = l.unwrap();
        let id = parser.parse_line(line);
        v.push(id);
        if id > max_id {
            max_id = id;
        }
    }

    println!("max ID is: {}", max_id);
    
    v.sort_unstable();

    for seats in v.windows(2) {
        if seats[0] + 2 == seats[1] {
            println!("seat: {}", seats[0] + 1);
        }
    }
}

struct Parser {}

impl Parser {
    fn new() -> Self {
        Parser {}
    }
    fn parse_line(&mut self, line: String) -> usize {
        let mut range = (0, 127);
        let mut seats_range = (0, 7);
        let mut rem = 127;
        let mut seats = 7;

        for (_, ch) in line.chars().enumerate() {
            match ch {
                'F' => {
                    let res = rem / 2;
                    range.1 -= res + 1;
                    rem = res;
                }
                'B' => {
                    let res = rem / 2;
                    range.0 += res + 1;
                    rem = res;
                }
                'R' => {
                    let res = seats / 2;
                    seats_range.0 += res + 1;
                    seats = res;
                }
                'L' => {
                    let res = seats / 2;
                    seats_range.1 -= res + 1;
                    seats = res;
                }

                _ => {}
            }
        }

        if range.0 != range.1 {
            println!("range not eq");
        }
        if seats_range.0 != seats_range.1 {
            println!("seats range not eq");
        }

        range.0 * 8 + seats_range.0
    }
}
