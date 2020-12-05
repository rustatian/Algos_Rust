#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test() {
        let mut f = File::open("data/day1.txt").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        let mut splitted: Vec<u32> = s.split('\n').map(|x| x.parse::<u32>().unwrap()).collect();

        let mut hm: HashMap<u32, bool> = HashMap::new();

        let year = 2020;
        for num in &splitted {
            let diff = year - num;
            if hm.contains_key(&num) {
                println!("the result is: {}", num * diff);
                break;
            } else {
                hm.insert(diff, true);
            }
        }

        println!("done part 1");
        println!("part 2 -------");

        splitted.sort_unstable();

        for (i, _) in splitted.iter().enumerate() {
            let mut left = i + 1;
            let mut right = splitted.len() - 1;

            while left < right {
                let curr = splitted[i] + splitted[right] + splitted[left];
                match curr.cmp(&year) {
                    Ordering::Greater => {
                        right -= 1;
                    }
                    Ordering::Less => {
                        left += 1;
                    }
                    Ordering::Equal => {
                        println!(
                            "the part2 result is: {}",
                            splitted[i] * splitted[right] * splitted[left]
                        );
                        break;
                    }
                }
            }
        }
    }
}

fn main() {

}