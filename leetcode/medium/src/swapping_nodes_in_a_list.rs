struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut h2 = head.clone();
        let mut h3 = head.clone();

        let mut val1 = 0;
        let mut num = 0;

        while h2.is_some() {
            match h2.take() {
                Some(v) => {
                    num += 1;
                    if num == k {
                        val1 = v.to_owned().val; 
                    }
                }
                None => {
                    break;
                }
            }
        }

        let mut count = num - k;

        while h3.is_some() {
            match h3.take() {
                Some(v) => {
                    num += 1;
                    if num == k {
                        val1 = v.to_owned().val; 
                    }
                }
                None => {
                    break;
                }
            }
        }


        head
    }
}

mod tests {
    #[test]
    fn test() {}
}
