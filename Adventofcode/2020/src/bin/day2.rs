#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};

    #[test]
    fn test() {
        let file = File::open("data/day2.txt").unwrap();

        let f = BufReader::new(file.try_clone().unwrap());

        let mut correct_num = 0;
        for line in f.lines() {
            let ll = line.unwrap();

            let mut iter = ll.split([':', '-', ' '].as_ref()).filter(|s| !s.is_empty());

            let left = iter.next().unwrap().parse::<usize>().unwrap();
            let right = iter.next().unwrap().parse::<usize>().unwrap();
            let letter = iter.next().unwrap().chars().next().unwrap();

            let count = ll.chars().filter(|&c| c == letter).count() - 1; // skip ident letter
            let correct: bool = left <= count && count <= right;
            if correct {
                correct_num += 1
            }
        }

        println!("correct: {}", correct_num);

        let file = File::open("data/day2.txt").unwrap();

        let f = BufReader::new(file);

        let mut correct_2 = 0;
        for line in f.lines() {
            let ll = line.unwrap();

            let mut iter = ll.split([':', '-', ' '].as_ref()).filter(|s| !s.is_empty());

            let left = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let right = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let letter = iter.next().unwrap().chars().next().unwrap();
            let w = iter.next().unwrap();

            let left_ch = w.chars().nth(left).unwrap();
            let right_ch = w.chars().nth(right).unwrap();

            // should be exactly one
            if left_ch == right_ch {
                continue;
            }

            if left_ch == letter || right_ch == letter {
                correct_2 += 1;
            }
        }

        println!("correct: {}", correct_2);
    }
}

fn main() {}