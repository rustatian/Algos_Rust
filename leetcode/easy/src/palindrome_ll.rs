/*
Given the head of a singly linked list, return true if it is a palindrome.
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution {}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut count = 0;
        let mut current = head.as_ref();

        // O(n) time to count
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        if count < 2 {
            return true;
        }
        let mut middle = (count as f32 / 2.0).floor() as i32;

        // O(n) time to split list to two halves
        let mut middle_counter = middle;
        let mut left = None;
        let mut right = head;
        while middle_counter > 0 {
            let pair = split_list(left, right);
            left = pair.0;
            right = pair.1;
            middle_counter -= 1;
        }
        // if odd number of nodes, then first node in the right head should be removed
        let right_head = if count % 2 == 1 {
            let (_, right_head) = split_list(None, right);
            right_head
        } else {
            right
        };

        // O(n) time to compare two halves
        let mut left_ptr = left.as_ref();
        let mut right_ptr = right_head.as_ref();
        while middle > 0 {
            let left_node = left_ptr.unwrap();
            let right_node = right_ptr.unwrap();
            if left_node.val != right_node.val {
                return false;
            }
            left_ptr = left_node.next.as_ref();
            right_ptr = right_node.next.as_ref();
            middle -= 1;
        }
        true
    }
}

fn split_list(
    left: Option<Box<ListNode>>,
    mut right: Option<Box<ListNode>>,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let right_tail = right.as_mut().and_then(|node| {
        let right_tail = node.next.take();
        node.next = left;
        right_tail
    });
    (right, right_tail)
}

/*
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        match (list1, list2) {
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(list1), Some(list2)) => {
                if list1.val >= list2.val {
                    Some(Box::new(ListNode {
                        val: list2.val,
                        next: Solution::merge_two_lists(Some(list1), list2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list1.val,
                        next: Solution::merge_two_lists(list1.next, Some(list2)),
                    }))
                }
            }

            (None, None) => None,
        }
    }
}
 */
