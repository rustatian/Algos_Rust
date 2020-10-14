use std::rc::Rc;

#[derive(Clone, Debug)]
struct BinaryTree {
    value: i32,
    left: Option<Rc<BinaryTree>>,
    right: Option<Rc<BinaryTree>>,
}

impl BinaryTree {
    fn new(value: i32, left: Option<Rc<BinaryTree>>, right: Option<Rc<BinaryTree>>) -> Self {
        BinaryTree { value, left, right }
    }

    fn branch_sums(root: &Option<Rc<BinaryTree>>) -> Vec<i32> {
        let mut v = vec![];
        let sum = 0;
        BinaryTree::branch_sums_helper(root, sum, &mut v);
        v
    }

    fn branch_sums_helper(nodes: &Option<Rc<BinaryTree>>, sum: i32, data: &mut Vec<i32>) {
        if nodes.is_none() {
            return;
        }

        let d = nodes.as_ref().unwrap();
        let new_sum = sum + d.value;
        if d.left.is_none() && d.right.is_none() {
            data.push(new_sum);
            return;
        }
        BinaryTree::branch_sums_helper(&d.left, new_sum, data);
        BinaryTree::branch_sums_helper(&d.right, new_sum, data);
    }
}

#[cfg(test)]
mod tests {
    use crate::branch_sums::BinaryTree;
    use std::rc::Rc;

    #[test]
    fn test() {
        let bt = BinaryTree::new(
            1,
            Option::from(Rc::from(BinaryTree::new(
                2,
                Option::from(Rc::from(BinaryTree::new(
                    4,
                    Option::from(Rc::from(BinaryTree::new(8, None, None))),
                    Option::from(Rc::from(BinaryTree::new(9, None, None))),
                ))),
                Option::from(Rc::from(BinaryTree::new(
                    5,
                    Option::from(Rc::from(BinaryTree::new(10, None, None))),
                    None,
                ))),
            ))),
            Option::from(Rc::from(BinaryTree::new(
                3,
                Option::from(Rc::from(BinaryTree::new(6, None, None))),
                Option::from(Rc::from(BinaryTree::new(7, None, None))),
            ))),
        );

        let res = BinaryTree::branch_sums(&Option::from(Rc::from(bt)));
        let expected = vec![15, 16, 18, 10, 11];
        assert_eq!(res, expected);
    }
}
