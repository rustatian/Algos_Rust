/*
A string can be abbreviated by replacing any number of non-adjacent, non-empty substrings with their lengths. The lengths should not have leading zeros.

For example, a string such as "substitution" could be abbreviated as (but not limited to):

"s10n" ("s ubstitutio n")
"sub4u4" ("sub stit u tion")
"12" ("substitution")
"su3i1u2on" ("su bst i t u ti on")
"substitution" (no substrings replaced)
The following are not valid abbreviations:

"s55n" ("s ubsti tutio n", the replaced substrings are adjacent)
"s010n" (has leading zeros)
"s0ubstitution" (replaces an empty substring)
Given a string word and an abbreviation abbr, return whether the string matches the given abbreviation.

A substring is a contiguous non-empty sequence of characters within a string.



Example 1:

Input: word = "internationalization", abbr = "i12iz4n"
Output: true
Explanation: The word "internationalization" can be abbreviated as "i12iz4n" ("i nternational iz atio n").
Example 2:

Input: word = "apple", abbr = "a2e"
Output: false
Explanation: The word "apple" cannot be abbreviated as "a2e".
 */

struct Solution {}

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let mut num = 0;
        let mut i = 0;
        let mut j = 0;

        while j<abbr.len() {
            if abbr.chars().nth(j).unwrap().is_numeric() {
                if abbr.chars().nth(j).unwrap() == '0' && num == 0 {
                    return false;
                }
                num = num*10+abbr.chars().nth(j).unwrap().to_string().parse::<i32>().unwrap();
            } else {
                i += num;
                num = 0;


                if word.len() <= i as usize {
                    return false;
                }

                if abbr.len() <= j as usize {
                    return false;
                }

                if i> word.len() as i32 || abbr.chars().nth(j).unwrap() != word.chars().nth(i as usize).unwrap() {
                    return false
                }
                i += 1;
            }
            j += 1;
        }

        num == (word.len() - i as usize) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::abbreviation::Solution;

    #[test]
    fn test() {
        let res =
            Solution::valid_word_abbreviation("hi".into(), "2i".into());
        assert!(res);
    }
}