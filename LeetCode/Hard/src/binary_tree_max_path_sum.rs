// Definition for A binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}


impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = std::i32::MIN;
        Solution::max_gain(&mut max_sum, root);
        max_sum
    }

    fn max_gain(max_sum: &mut i32, node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            *max_sum = 0;
            return;
        }
    }
}