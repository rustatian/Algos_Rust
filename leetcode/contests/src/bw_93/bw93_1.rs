struct Solution {}

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut max = i32::MIN;
        for s in strs {
            if s.chars().all(|c| c.is_ascii_digit()) {
                if s.parse::<i32>().unwrap() > max {
                    max = s.parse::<i32>().unwrap();
                };
                continue;
            }
            if s.len() as i32 > max {
                max = s.len() as i32;
            }
        }
        max
    }
}

mod tests {
    use crate::bw_93::bw93_1::Solution;

    #[test]
    fn test() {
        assert_eq!(
            5,
            Solution::maximum_value(vec![
                String::from("alic3"),
                String::from("bob"),
                String::from("3"),
                String::from("4"),
                String::from("00000")
            ])
        );

        assert_eq!(
            1,
            Solution::maximum_value(vec![
                String::from("1"),
                String::from("01"),
                String::from("001"),
                String::from("0001"),
            ])
        );
    }
}
