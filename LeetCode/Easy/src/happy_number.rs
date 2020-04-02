// Write an algorithm to determine if a number is "happy".
//
// A happy number is a number defined by the following process: Starting with any positive integer,
// replace the number by the sum of the squares of its digits, and repeat the process until
// the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy numbers.
//
// Example:
//
// Input: 19
// Output: true
// Explanation:
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1

pub struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashMap;
        // the main point of using hash map here is to prevent recursion
        // for number 19 this is not necessary as it unwinds to 1
        // but for number 2 for example, we will see the following:
        // 2->4->16->37->58->89->145->42->20->4
        // we already know all the chain which start from 4
        // so, this is infinite loop
        // with map we check if there is number duplicates or not
        let mut h: HashMap<i32, bool> = HashMap::new();
        let calculator = |x: String| x.chars().fold(0, |acc, x| { acc + x.to_digit(10).unwrap().pow(2) }) as i32;

        let mut rr = calculator(n.to_string());
        while rr > 1 {
            rr = calculator(rr.to_string());
            // check the recursion
            match h.get(&rr) {
                None => {
                    h.insert(rr, true);
                    continue;
                },
                Some(_) => {
                    return false;
                },
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_happy(19), true);
    assert_eq!(Solution::is_happy(2), false);
}