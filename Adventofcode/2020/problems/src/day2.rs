#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};

    #[test]
    fn test() {
        let file = File::open("data/day2.txt").unwrap();

        let f = BufReader::new(file);

        let mut incorrect = 0;
        for line in f.lines() {
            let ll = line.unwrap();

            let mut iter = ll.split([':', '-', ' '].as_ref()).filter(|s| !s.is_empty());

            let left = iter.next().unwrap().parse::<usize>().unwrap();
            let right = iter.next().unwrap().parse::<usize>().unwrap();
            let letter = iter.next().unwrap().chars().next().unwrap();

            let count = ll.chars().filter(|&c| c == letter).count() - 1;
            let correct: bool = left <= count && count <= right;
            if correct {
                incorrect += 1
            }
        }

        println!("incorrect: {}", incorrect);
    }
}
