struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
    fn from(val: TreeNode) -> Self {
        Option::from(Rc::new(RefCell::new(val)))
    }
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
    pub const MOD: i64 = 1000000007;

    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = vec![];
        let total_sum = Solution::sum(root, &mut sums);
        let mut max: i64 = 0;

        for s in sums {
            max = std::cmp::max(max, s * (total_sum - s));
        }

        (max % Solution::MOD) as i32
    }

    pub fn sum(node: Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) -> i64 {
        match node {
            None => 0,
            Some(val) => {
                let v = val.as_ref().borrow();

                let sl = Solution::sum(v.left.clone(), sums);
                let sr = Solution::sum(v.right.clone(), sums);

                let total_sum = (v.val as i64) + sl + sr;

                sums.push(total_sum);

                total_sum
            }
        }
    }
}

mod tests {
    use crate::maximum_product_of_splitted_bt::{Solution, TreeNode};

    #[test]
    fn test() {
        let mut tn = TreeNode::new(1);
        tn.left = TreeNode::new(2).into();
        tn.right = TreeNode::new(3).into();

        assert_eq!(9, Solution::max_product(tn.into()));
    }
}
