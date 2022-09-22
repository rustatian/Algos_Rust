
struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        use std::cmp::Ordering;

        let mut start = 0;
        let mut end = letters.len();

        while start < end {
            let middle = start + (end - start) /2;

            match letters[middle].cmp(&target){
                Ordering::Greater => {
                    end = middle;
                }
                _ => {
                    start = middle + 1
                }
            }
        }

        if start == letters.len() {
            return letters[0]
        }
        letters[start]
    }
}

mod tests {
    use crate::find_smallest_letter::Solution;

    #[test]
    fn test() {
        assert_eq!(
            'c',
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j')
        );
    }
}
