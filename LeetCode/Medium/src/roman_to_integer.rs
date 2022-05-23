use std::collections::VecDeque;

struct Solution {}

/*
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

I can be placed before V (5) and X (10) to make 4 and 9.
X can be placed before L (50) and C (100) to make 40 and 90.
C can be placed before D (500) and M (1000) to make 400 and 900.
Given a roman numeral, convert it to an integer.
 */
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res: i32 = 0;
        let mut v: VecDeque<char> = s.chars().collect();

        while let Some(ch) = v.pop_front() {
            // handle IV + IX
            // if cur element not the last
            if let Some(val) = v.front() {
                // MM case or XI case
                if symbol_to_int(ch) >= symbol_to_int(*val) {
                    res += symbol_to_int(ch);
                    continue;
                    // case like IX
                    // I - ch
                    // X - val
                } else if symbol_to_int(ch) < symbol_to_int(*val) {
                    res += symbol_to_int(v.pop_front().unwrap()) - symbol_to_int(ch);
                    continue;
                }
            } else {
                res += symbol_to_int(ch);
                continue;
            }
        }
        res
    }
}

fn symbol_to_int(chr: char) -> i32 {
    match chr {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => {
            panic!("unknown chan");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::roman_to_integer::Solution;

    #[test]
    fn test_roman() {
        // let s = Solution::roman_to_int("III".into());
        // assert_eq!(s, 3);
        let s = Solution::roman_to_int("MMCMLXXXIX".into());
        assert_eq!(s, 2989);
        let s = Solution::roman_to_int("IV".into());
        assert_eq!(s, 4);
    }
}
