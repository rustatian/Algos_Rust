use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_some() || root2.is_none() && root1.is_some() {
            return false;
        }

        let r1 = Solution::iter(root1);
        let r2 = Solution::iter(root2);

        r1.eq(&r2)
    }

    pub fn iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        stack.push(root);
        let mut vals = vec![];

        while let Some(val) = stack.pop() {
            match val {
                None => {
                    return vals;
                }
                Some(vval) => {
                    let v = vval.as_ref().borrow();
                    if v.left.is_none() && v.right.is_none() {
                        vals.push(v.val);
                        continue;
                    }

                    if v.left.is_some() {
                        stack.push(v.left.clone());
                    }

                    if v.right.is_some() {
                        stack.push(v.right.clone());
                    }
                }
            }
        }
        vals
    }
}
