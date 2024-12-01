use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("./data/day_1.txt").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let splitted: Vec<_> = s.split('\n').collect();
    let mut res = 0;
    let mut v1: Vec<i32> = vec![];

    let mut mm: HashMap<i32, i32> = HashMap::new();

    for v in &splitted {
        let split: Vec<_> = v.split("   ").collect();
        if split.len() != 2 {
            continue;
        }

        let n1 = split[0].parse::<i32>().unwrap();
        let n2 = split[1].parse::<i32>().unwrap();

        *mm.entry(n2).or_insert(0) += 1;

        v1.push(n1);
    }

    let mut res: u64 = 0;

    for v in &v1 {
        if mm.contains_key(&v) {
            let t = (*v as u64) * *mm.get(&v).unwrap() as u64;
            res += t;
        }
    }

    println!("{}", res);
}
