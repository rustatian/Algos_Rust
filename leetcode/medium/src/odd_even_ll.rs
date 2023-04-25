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

        let mut odd = Box::new(ListNode::new(0));
        let mut even = Box::new(ListNode::new(0));

        let mut ot = &mut odd;
        let mut et = &mut even;

        let mut even_odd = true;

        while let Some(mut node) = head {
            head = node.next.take();

            match even_odd {
                true => {
                    ot.next = Some(node);
                    ot = ot.next.as_mut()?;
                }
                false => {
                    et.next = Some(node);
                    et = et.next.as_mut()?;
                }
            }

            // reverse
            even_odd = !even_odd;
        }

        ot.next = even.next;
        odd.next
    }
}
