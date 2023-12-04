struct Solution {}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let iter = s.as_bytes().windows(k as usize);
        let mut curr_max = 0;

        // check the current maximum
        for i in 0..k as usize {
            match s.as_bytes()[i] {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    curr_max += 1;
                }
                _ => {}
            }
        }

        // max is eq to k, can't be more
        if curr_max == k {
            return curr_max;
        }

        let initial_max = curr_max;
        let mut local_max = curr_max;
        for i in 1..s.len() {
            if (i + k as usize) >= s.len() {
                break;
            }
            // char to be removed (past one)
            let rem = s.as_bytes()[i - 1];
            let add = s.as_bytes()[(i + k as usize) - 1];

            match rem {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    local_max -= 1;
                }
                _ => {}
            }

            match add {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    local_max += 1;
                }
                _ => {}
            }

            if local_max > curr_max {
                curr_max = local_max;
            }
        }

        if initial_max > curr_max {
            return initial_max;
        }

        curr_max
    }
}

// 'a', 'e', 'i', 'o', and 'u'.

mod tests {
    use crate::max_number_vowels::Solution;

    #[test]
    fn test() {
        // assert_eq!(2, Solution::max_vowels("leetcode".to_string(), 3));
        assert_eq!(3, Solution::max_vowels("abciiidef".to_string(), 3));
    }
}
