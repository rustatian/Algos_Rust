struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = "aeiouAEIOU";

        let mut sorted_s = s
            .chars()
            .filter(|c| vowels.contains(*c))
            .collect::<Vec<char>>();

        sorted_s.sort_unstable();
        let mut res = String::new();
        let mut vowel_idx = 0;

        for c in s.chars() {
            if vowels.contains(c) {
                res.push(sorted_s[vowel_idx]);
                vowel_idx += 1;
            } else {
                res.push(c);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::sort_vowels::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::sort_vowels("lEetcOde".to_string()),
            "lEOtcede".to_string()
        );
        assert_eq!(
            Solution::sort_vowels("lYmpH".to_string()),
            "lYmpH".to_string()
        );
    }
}
