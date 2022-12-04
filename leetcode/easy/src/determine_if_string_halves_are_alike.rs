struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let ss: Vec<char> = s.chars().into_iter().collect();
        let mut rpc = 0;
        let mut lpc = 0;

        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            match ss[start] {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    rpc += 1;
                }

                _ => {}
            }

            match ss[end] {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    lpc += 1;
                }

                _ => {}
            }

            start += 1;
            end -= 1;
        }

        rpc == lpc
    }
}

mod tests {
    use crate::determine_if_string_halves_are_alike::Solution;

    #[test]
    fn test() {
        assert!(Solution::halves_are_alike(String::from("book")));
        assert!(!Solution::halves_are_alike(String::from("textbook")));
    }
}
