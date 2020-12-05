#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test() {
        let f = File::open("data/day4.txt").unwrap();

        let dr = BufReader::new(f);
        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut hgt = false;
        let mut hcl = false;
        let mut ecl = false;
        let mut pid = false;
        //let mut cid = false;

        let mut valid = 0;
        for l in dr.lines() {
            let line = l.unwrap();
            if line == "" {
                if !byr || !iyr || !eyr || !hgt || !hcl || !ecl || !pid {
                    byr = false;
                    iyr = false;
                    eyr = false;
                    hgt = false;
                    hcl = false;
                    ecl = false;
                    pid = false;
                    //cid = false;
                    continue;
                } else {
                    valid += 1;
                    byr = false;
                    iyr = false;
                    eyr = false;
                    hgt = false;
                    hcl = false;
                    ecl = false;
                    pid = false;
                    //cid = false;
                    continue;
                }
            }

            if line.contains("byr:") {
                byr = true;
            }
            if line.contains("iyr:") {
                iyr = true;
            }
            if line.contains("eyr:") {
                eyr = true;
            }
            if line.contains("hgt:") {
                hgt = true;
            }
            if line.contains("hcl:") {
                hcl = true;
            }
            if line.contains("ecl:") {
                ecl = true;
            }
            if line.contains("pid:") {
                pid = true;
            }
            // if line.contains("cid") {
            //     cid = true;
            // }
        }

        println!("valid: {}", valid);
    }
}
