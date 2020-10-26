//You are given a string s containing lowercase English letters, and a matrix shift, where shift[i] = [direction, amount]:
//
//    direction can be 0 (for left shift) or 1 (for right shift).
//    amount is the amount by which string s is to be shifted.
//    A left shift by 1 means remove the first character of s and append it to the end.
//    Similarly, a right shift by 1 means remove the last character of s and add it to the beginning.
//
//Return the final string after all operations.
//
//
//
//Example 1:
//
//Input: s = "abc", shift = [[0,1],[1,2]]
//Output: "cab"
//Explanation:
//[0,1] means shift to left by 1. "abc" -> "bca"
//[1,2] means shift to right by 2. "bca" -> "cab"
//
//Example 2:
//
//Input: s = "abcdefg", shift = [[1,1],[1,1],[0,2],[1,3]]
//Output: "efgabcd"
//Explanation:
//[1,1] means shift to right by 1. "abcdefg" -> "gabcdef"
//[1,1] means shift to right by 1. "gabcdef" -> "fgabcde"
//[0,2] means shift to left by 2. "fgabcde" -> "abcdefg"
//[1,3] means shift to right by 3. "abcdefg" -> "efgabcd"
//
//
//
//Constraints:
//
//    1 <= s.length <= 100
//    s only contains lower case English letters.
//    1 <= shift.length <= 100
//    shift[i].length == 2
//    0 <= shift[i][0] <= 1
//    0 <= shift[i][1] <= 100
//
struct Solution {}

impl Solution {
    pub fn string_shift_2(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut r = 0;
        let mut l = 0;

        // calculate operations
        // 0 - left
        // 1 - right
        for v in shift {
            if v[0] == 0 {
                l += v[1];
            } else {
                r += v[1];
            }
        }


        let mut st = s.clone();
        // Calculate direction
        let res = l - r;
        if res < 0 {
            l = 0;
            r = res.abs();

            if r > st.len() as i32 {
                while r > st.len() as i32 {
                    r -= st.len() as i32;
                }
            }

            let (a, b) = st.split_at(st.len() - r as usize);


            return format!("{}{}", b.to_string(), a.to_string());
        } else if r > 0 {
            r = 0;
            l = res;

            if l > st.len() as i32 {
                while l > st.len() as i32 {
                    l -= st.len() as i32;
                }
            }

            let (a, b) = st.split_at(l as usize);


            return format!("{}{}", b.to_string(), a.to_string());
        } else {
            s
        }
    }
    // RUST WAY
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let len = s.chars().count() as i32;

        let sh_sum: i32 = shift.iter().map(
            |v| if v[0] == 0 {
                -v[1]
            } else {
                v[1]
            }
        ).sum();

        let (s1, s2) = s.split_at(((len - sh_sum % len) % len) as usize);
        format!("{}{}", s2, s1)
    }
}


#[test]
fn tests() {
    assert_eq!(Solution::string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]]), "cab".to_string());
    assert_eq!(Solution::string_shift("abcdefg".to_string(), vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]), "efgabcd".to_string());
    assert_eq!(Solution::string_shift("mecsk".to_string(), vec![vec![1, 4], vec![0, 5], vec![0, 4], vec![1, 1], vec![1, 5]]), "kmecs".to_string());
    assert_eq!(Solution::string_shift("wpdhhcj".to_string(), vec![vec![0, 7], vec![1, 7], vec![1, 0], vec![1, 3], vec![0, 3], vec![0, 6], vec![1, 2]]), "hcjwpdh".to_string());
    assert_eq!(Solution::string_shift("yisxjwry".to_string(), vec![vec![1, 8], vec![1, 4], vec![1, 3], vec![1, 6], vec![0, 6], vec![1, 4], vec![0, 2], vec![0, 1]]), "yisxjwry".to_string());
}
