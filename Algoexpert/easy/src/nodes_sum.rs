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

    fn nodes_sums(root: &Option<Rc<BinaryTree>>) -> i32 {
        let depth = 0;
        let mut sum = 0;
        BinaryTree::nodes_sums_helper(root, depth, &mut sum);
        sum
    }

    fn nodes_sums_helper(nodes: &Option<Rc<BinaryTree>>, depth: i32, sum: &mut i32) {
        if nodes.is_none() {
            return;
        }

        *sum += depth;
        let d = nodes.as_ref().unwrap();
        if d.left.is_none() && d.right.is_none() {
            return;
        }

        BinaryTree::nodes_sums_helper(&d.left, depth + 1, sum);
        BinaryTree::nodes_sums_helper(&d.right, depth + 1, sum);
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes_sum::BinaryTree;
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
                Option::from(Rc::from(BinaryTree::new(5, None, None))),
            ))),
            Option::from(Rc::from(BinaryTree::new(
                3,
                Option::from(Rc::from(BinaryTree::new(6, None, None))),
                Option::from(Rc::from(BinaryTree::new(7, None, None))),
            ))),
        );

        let res = BinaryTree::nodes_sums(&Option::from(Rc::from(bt)));
        let expected = 16;
        assert_eq!(res, expected);
    }
}
