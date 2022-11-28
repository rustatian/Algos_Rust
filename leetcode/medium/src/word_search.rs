struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        true
    }

    pub fn check() -> bool {
        true
    }
}

mod tests {
    use crate::word_search::Solution;

    #[test]
    fn test() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".into()
        ));
    }
}
