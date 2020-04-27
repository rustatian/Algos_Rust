use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut t1: Vec<char> = text1.chars().collect();
        let mut t2: Vec<char> = text2.chars().collect();

        if t2.len() < t1.len() {
            std::mem::swap(&mut t1, &mut t2);
        }

        let mut prev: Vec<i32> = vec![0; t1.len() + 1];

        for (col, _) in t2.iter().enumerate().rev() {
            let mut cur: Vec<i32> = vec![0; t2.len() + 1];

            for (row, _) in t1.iter().enumerate().rev() {
                if t1[row] == t2[col] {
                    cur[row] = 1 + prev[row + 1];
                } else {
                    cur[row] = std::cmp::max(prev[row], cur[row + 1]);
                }
            }
            prev = cur;
        }


        prev[0]
    }
}

//Try dynamic programming. DP[i][j] represents the longest common subsequence of text1[0 ... i] & text2[0 ... j].
//DP[i][j] = DP[i - 1][j - 1] + 1 , if text1[i] == text2[j] DP[i][j] = max(DP[i - 1][j], DP[i][j - 1]) , otherwise

#[test]
fn tests() {
    assert_eq!(Solution::longest_common_subsequence("oxcpqrsvwf".to_string(), "shmtulqrypy".to_string()), 2);
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
}