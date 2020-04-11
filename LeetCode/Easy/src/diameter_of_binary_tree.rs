// Given a binary tree, you need to compute the length of the diameter of the tree.
// The diameter of a binary tree is the length of the longest path between any two nodes in a tree.
// This path may or may not pass through the root.
//
// Example:
// Given a binary tree
//
//       1
//      / \
//     2   3
//    / \
//   4   5
//
// Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
//
// Note: The length of path between two nodes is represented by the number of edges between them.

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

use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // we need to check (because receiving the Option)
        if root.is_some() {
            // recursion
            let (_, max_diameter) = Solution::calculate_diameter(&root);
            max_diameter
        } else {
            0
        }
    }

    fn calculate_diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();

            let (d_left, diam_left) = Solution::calculate_diameter(&node.left);
            let (d_right, diam_right) = Solution::calculate_diameter(&node.right);

            let max_depth = 1 + std::cmp::max(d_left, d_right);
            let max_diameter = *[diam_left, diam_right, d_left + d_right]
                .iter()
                .max()
                .unwrap();

            (max_depth, max_diameter)
        } else {
            (0, 0)
        }
    }
}