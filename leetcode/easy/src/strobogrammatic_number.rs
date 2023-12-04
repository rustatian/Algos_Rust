struct Solution {}

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        use std::collections::HashMap;

        let bts = num.as_bytes();
        let mut hm = HashMap::new();
        hm.insert('0', '0');
        hm.insert('1', '1');
        hm.insert('8', '8');

        if num.len() == 1 {
            if !hm.contains_key(&(bts[0] as char)) {
                return false;
            }
            return true;
        }

        hm.insert('6', '9');
        hm.insert('9', '6');

        let mut start = 0;
        let mut end = num.len() - 1;

        while start <= end {
            let n = bts[end] as char;
            if !hm.contains_key(&n) {
                return false;
            }

            let rot = *hm.get(&n).unwrap();

            if rot != bts[start] as char {
                return false;
            }

            start += 1;
            end -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::strobogrammatic_number::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_strobogrammatic(String::from("69")));
    }
}
