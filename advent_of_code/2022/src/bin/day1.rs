use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("advent_of_code/2022/src/bin/data/input_1.txt").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let splitted: Vec<_> = s.split('\n').collect();

    let mut num = 0;
    let mut sum = 0;

    for v in &splitted {
        if v.is_empty() {
            if sum > num {
                num = sum;
            }
            sum = 0;
            continue;
        }
        sum += v.parse::<i32>().unwrap();
    }

    println!("num: {}", num);

    // part 2

    let mut all = Vec::new();

    for v in &splitted {
        if v.is_empty() {
            all.push(sum);
            sum = 0;
            continue;
        }
        sum += v.parse::<i32>().unwrap();
    }

    all.sort_unstable();

    println!("top3: {}", all[all.len() - 1] + all[all.len() - 2] + all[all.len() - 3])
}
