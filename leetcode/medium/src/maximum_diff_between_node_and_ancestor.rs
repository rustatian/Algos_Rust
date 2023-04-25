struct Solution {}

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

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let r = root.clone().unwrap().as_ref().borrow().val;
        let mut res_max = 0;

        Solution::dfs(root, r, r, &mut res_max);
        res_max
    }

    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max: i32, min: i32, res: &mut i32) {
        match node {
            None => {}
            Some(n) => {
                let v = n.as_ref().borrow();
                let val = v.val;

                let mx = i32::abs(max - val);
                let mn = i32::abs(min - val);

                let tmp = std::cmp::max(mx, mn);
                *res = std::cmp::max(tmp, *res);

                Solution::dfs(
                    v.left.clone(),
                    std::cmp::max(max, v.val),
                    std::cmp::min(min, v.val),
                    res,
                );
                Solution::dfs(
                    v.right.clone(),
                    std::cmp::max(max, v.val),
                    std::cmp::min(min, v.val),
                    res,
                );
            }
        }
    }
}
