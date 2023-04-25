use std::cell::RefCell;
use std::rc::Rc;

/*
Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].

Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
Output: 32
Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
 */
struct Solution {}

// Definition for a binary tree node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Solution::dfs(root, low, high)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let mut sum = 0;

        match node {
            None => 0,
            Some(val) => {
                let nd = val.as_ref().borrow();
                if nd.val >= low && nd.val <= high {
                    sum += nd.val;
                }

                sum += Solution::dfs(nd.left.clone(), low, high);
                sum += Solution::dfs(nd.right.clone(), low, high);

                sum
            }
        }
    }
}
