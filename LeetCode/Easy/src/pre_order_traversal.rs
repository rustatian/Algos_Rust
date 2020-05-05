// Definition for a binary tree node.
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

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }






        vec![]
    }

    fn recursion() {

    }
}

#[test]
fn tests() {
    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    right.left = Option::from(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right = Option::from(Rc::new(RefCell::new(right)));

    assert_eq!(Solution::preorder_traversal(Option::from(Rc::new(RefCell::new(root)))), vec![1, 2, 3]);
}