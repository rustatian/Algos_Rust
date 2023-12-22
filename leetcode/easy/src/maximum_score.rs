struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let mut max = 0;

        for i in 1..s.len() {
            let first = &s[0..i];
            let second = &s[i..=s.len() - 1];

	    let mut zeroes = 0;
	    for i in first {
		if *i == b'0' {
			zeroes+=1;
		}
	    }

	    let mut ones = 0;
	    for i in second {
		if *i == b'1' {
			ones+=1;
		}
	    }

            max = std::cmp::max(max, zeroes + ones);
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(5, Solution::max_score("011101".to_string()));
    }
}
