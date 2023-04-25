struct Solution{}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack:Vec<String> = vec![];

        let mut parts = path.split('/');
        for part in parts {
            match part {
                "." | "" => {
                    continue;
                }
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }

                }
                _ => {
                    stack.push(part.to_string());
                }
            }
        }

        let mut res = String::new();

        for part in stack {
            res.push_str("/");
            res.push_str(&part);
        }
        
        res
    }
}

mod tests {
    use crate::simplify_path::Solution;

    #[test]
    fn test(){
        assert_eq!(String::from("/home"), Solution::simplify_path(String::from("/home/")));
        assert_eq!(String::from("/"), Solution::simplify_path(String::from("/../")));
    }
}
