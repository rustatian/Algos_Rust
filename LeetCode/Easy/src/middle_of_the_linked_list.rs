// Given a non-empty, singly linked list with head node head, return a middle node of linked list.
//
// If there are two middle nodes, return the second middle node.
//
//
//
// Example 1:
//
// Input: [1,2,3,4,5]
// Output: Node 3 from this list (Serialization: [3,4,5])
// The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
// Note that we returned a ListNode object ans, such that:
// ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
//
// Example 2:
//
// Input: [1,2,3,4,5,6]
// Output: Node 4 from this list (Serialization: [4,5,6])
// Since the list has two middle nodes with values 3 and 4, we return the second one.
//
//
//
// Note:
//
// The number of nodes in the given list will be between 1 and 100.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Here is very simple solution
        // we initialize 2 pointers, fast and slow
        // fast is pointing to every 2-nd node
        // slow is iterating every node
        // imagine we have ListNodes with values 1 2 3 4 5 6
        // at iteration 1: slow - 1, fast - 2
        // at iteration 2: slow - 2, fast - 4
        // at iteration 3: slow - 3, fast - 6
        // at iteration 4: slow - 4, fast - None (end)
        // and we return slow, it will be 4,5,6
        let mut slow_ptr = head.clone();
        let mut fast_ptr = head.clone();

        if head != None {
            loop {
                if fast_ptr.is_some() && fast_ptr.clone().unwrap().next.is_some() {
                    fast_ptr = fast_ptr.unwrap().next.unwrap().next;
                    slow_ptr = slow_ptr.unwrap().next;
                } else {
                    break
                }

            }
        }

        slow_ptr
    }
}

#[test]
fn tests() {
    let mut v1 = ListNode::new(1);
    let mut v2 = ListNode::new(2);
    let mut v3 = ListNode::new(3);
    let mut v4 = ListNode::new(4);
    let mut v5 = ListNode::new(5);
    let v6 = ListNode::new(6);

    v5.next = Option::from(Box::from(v6));
    v4.next = Option::from(Box::from(v5));
    v3.next = Option::from(Box::from(v4.clone()));
    v2.next = Option::from(Box::from(v3));
    v1.next = Option::from(Box::from(v2));

    assert_eq!(Solution::middle_node(Option::from(Box::from(v1))), Option::from(Box::from(v4)))
}