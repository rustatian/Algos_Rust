// Definition for singly-linked list.
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

struct Solution {}

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let mut odd = ListNode::new(0);
        let mut even = ListNode::new(0);

        let mut odd_even = true;
        let mut curr = head.clone();

        while let Some(node) = curr.take() {
           match odd_even {
               true => {

               }

               false => {

               }
           }

           odd_even = !odd_even;
        }
        
        curr
    }
}

#[cfg(test)]
mod tests {
    use crate::odd_even_ll::ListNode;

    #[test]
    fn test1(){
    }
}
