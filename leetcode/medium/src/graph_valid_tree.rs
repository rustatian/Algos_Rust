struct Solution {}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if edges.len() != (n - 1) as usize {
            return false;
        }

        let mut root = vec![0; n as usize];

        for i in 0..n {
            root[i as usize] = i
        }

        for edge in &edges {
            if !Solution::union(edge[0], edge[1], &mut root) {
                return false;
            }
        }

        true
    }

    pub fn union(x: i32, y: i32, root: &mut Vec<i32>) -> bool {
        let root_x = Solution::find(x, root);
        let root_y = Solution::find(y, root);

        if root_x == root_y {
            return false;
        }

        root[root_x as usize] = root_y;

        true
    }

    pub fn find(x: i32, root: &mut Vec<i32>) -> i32 {
        if x == root[x as usize] {
            return x;
        }

        root[x as usize] = Solution::find(root[x as usize], root);
        root[x as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::graph_valid_tree::Solution;

    #[test]
    fn test() {
        assert!(Solution::valid_tree(
            5,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]
        ))
    }
}
