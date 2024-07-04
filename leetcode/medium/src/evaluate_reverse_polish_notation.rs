struct Solution {}

impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        let mut sum = 0;
        while let Some(val) = tokens.pop() {

            match val.as_ref() {
                "+" => {
                    let n1 = tokens.pop().unwrap();
                    let n2 = tokens.pop().unwrap();
                }
                "*" => {}
                _ => {}
            }
        }
        1
    }
}

mod tests {
    use crate::evaluate_reverse_polish_notation::Solution;

    #[test]
    fn test() {
        assert_eq!(
            9,
            Solution::eval_rpn(
                vec!["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        );
        assert_eq!(
            6,
            Solution::eval_rpn(
                vec!["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        );
        assert_eq!(
            22,
            Solution::eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        );
    }
}
