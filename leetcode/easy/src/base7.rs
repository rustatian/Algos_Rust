struct Solution {}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }

        // direct conv
        if num > -7 && num < 7 {
            return String::from(num.to_string());
        }
        let mut res = String::new();

        let mut nn = if num < 0 { num*-1 } else {num};

        while nn != 0 {
            let tmp = nn % 7;
            nn = nn / 7;
            res.push_str(&tmp.to_string());
        }

        if num < 0 {
            res.push('-');
        }

        res.chars().rev().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::base7::Solution;

    #[test]
    fn test(){
        assert_eq!(String::from("202"), Solution::convert_to_base7(100));
        assert_eq!(String::from("-10"), Solution::convert_to_base7(-7));
    }
}
