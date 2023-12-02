use std::{fs::File, io::Read};

fn main() {
    // Your code here
    let mut f = File::open("advent_of_code/2023/src/bin/data/input_2.txt").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let splitted: Vec<_> = s.split('\n').collect();
    let mut res = 0;

    for row in &splitted {
        let mut red = -100000;
        let mut green = -100000;
        let mut blue = -100000;

        let second = row.split_at(row.find(':').unwrap() + 1).1;

        // 1 green, 3 red, 6 blue; 1 red, 2 green, 3 blue; 1 blue, 2 red, 3 green
        let iter = second.split(';');
        // 3 blue, 4 red, 1 green
        for y in iter {
            // 3 blue
            let yy = y.split(',');

            // 3 blue
            for ii in yy {
                if ii.contains("red") {
                    red = std::cmp::max(
                        ii.trim_start().split(' ').collect::<Vec<_>>()[0]
                            .parse::<i32>()
                            .unwrap(),
                        red,
                    );
                } else if ii.contains("green") {
                    green = std::cmp::max(
                        ii.trim_start().split(' ').collect::<Vec<_>>()[0]
                            .parse::<i32>()
                            .unwrap(),
                        green,
                    );
                } else if ii.contains("blue") {
                    blue = std::cmp::max(
                        ii.trim_start().split(' ').collect::<Vec<_>>()[0]
                            .parse::<i32>()
                            .unwrap(),
                        blue,
                    );
                }
            }
        }

        res += red * green * blue;
    }

    println!("{}", res);
}
