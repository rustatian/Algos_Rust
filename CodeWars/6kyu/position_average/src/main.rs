fn main() {
    println!("Hello, world!");
}

//Suppose you have 4 numbers: '0', '9', '6', '4' and 3 strings composed with them:
//
//s1 = "6900690040"
//s2 = "4690606946"
//s3 = "9990494604"
//
//Compare s1 and s2 to see how many positions they have in common: 0 at index 3, 6 at index 4, 4 at index 8 ie 3 common positions out of ten.
//
//Compare s1 and s3 to see how many positions they have in common: 9 at index 1, 0 at index 3, 9 at index 5 ie 3 common positions out of ten.
//
//Compare s2 and s3. We find 2 common positions out of ten.
//
//So for the 3 strings we have 8 common positions out of 30 ie 0.2666... or 26.666...%
//
//Given n substrings (n >= 2) in a string s our function pos_average will calculate the average percentage of positions that are the same between the (n * (n-1)) / 2 sets of substrings taken amongst the given n substrings. It can happen that some substrings are duplicate but since their ranks are not the same in s they are considered as different substrings.
//
//The function returns the percentage formatted as a float with 10 decimals but the result is tested at 1e.-9 (see function assertFuzzy in the tests).
//Example:
//
//Given string s = "444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490" composing a set of n = 10 substrings (hence 45 combinations), pos_average returns 29.2592592593.
//
//In a set the n substrings will have the same length ( > 0 ).
//Notes
//
//You can see other examples in the "Sample tests".

fn pos_average(s: &str) -> f64 {
    let v: Vec<&str> = s.split(",").collect();

    for i in 0..v.len() {
        for j in 0..v[i].len() {
            println!("{}", v[i])


        }
//        println!("{}", v[i])
    }
    1.0
    // your code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy(s: &str, expected: f64) {
        println!("{}", s);
        let merr = 1.0e-9;
        let actual = pos_average(s);
        let inrange =
            if expected == 0.0 {
                (actual.abs() <= merr)
            } else {
                ((actual - expected).abs() / expected <= merr)
            };
        if inrange == false {
            println!("Expected value must be near: {:e} but was:{:e}", expected, actual);
        } else {
            println!("....... correct");
        }
        assert_eq!(inrange, true);
        println!("{}", inrange);
    }

    #[test]
    fn basic_tests() {
        assert_fuzzy("466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096", 26.6666666667);
        assert_fuzzy("444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490", 29.2592592593);
    }
}