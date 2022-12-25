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

use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Solution::max_gain(&mut max_sum, root);
        max_sum
    }

    fn max_gain(max_sum: &mut i32, node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(val) => {
                let v = val.as_ref().borrow();

                let l = std::cmp::max(Solution::max_gain(max_sum, v.left.clone()), 0);
                let r = std::cmp::max(Solution::max_gain(max_sum, v.right.clone()), 0);

                let n = v.val + l + r;
                *max_sum = std::cmp::max(*max_sum, n);

                v.val + std::cmp::max(l, r)
            }
        }
    }
}
