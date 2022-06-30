use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();

        for (i, ch) in s.chars().enumerate() {
            match hm.entry(ch) {
                Occupied(mut en) => {
                    *en.get_mut() += 1;
                }
                Vacant(en) => {
                    en.insert(1);
                }
            }
        }

        for (i, ch) in s.chars().enumerate() {
            if let Some(val) = hm.get(&ch) {
                if *val == 1 {
                    return i as i32;
                }
            }
        }

        -1
    }

    pub fn first_uniq_char2(s: String) -> i32 {
        // english alphabet, lowercase
        let mut slice = [0; 26];

        // assign a number
        for char in s.chars() {
            slice[(char as u8 - b'a') as usize] += 1;
        }

        for (i, char) in s.chars().enumerate() {
            if slice[(char as u8 - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

mod tests {
    use crate::first_unique_char::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::first_uniq_char(String::from("leetcode")));
    }
}
