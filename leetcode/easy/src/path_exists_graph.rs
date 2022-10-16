struct Solution {}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut root = vec![0; n as usize];
        let mut rank = vec![0; n as usize];

        for i in 0..n {
            root[i as usize] = i;
            rank[i as usize] = 0;
        }

        for v in &edges {
            Solution::union(v[0], v[1], &mut root, &mut rank);
        }

        Solution::find(source, &mut root) == Solution::find(destination, &mut root)
    }

    pub fn find(x: i32, root: &mut [i32]) -> i32 {
        if root[x as usize] == x {
            return x;
        }

        root[x as usize] = Solution::find(root[x as usize], root);

        root[x as usize]
    }

    pub fn union(x: i32, y: i32, root: &mut [i32], rank: &mut [i32]) {
        let x_res = Solution::find(x, root);
        let y_res = Solution::find(y, root);

        if x_res == y_res {
            return;
        }

        use std::cmp::Ordering;

        match rank[x_res as usize].cmp(&rank[y_res as usize]) {
            Ordering::Less => root[x_res as usize] = y_res,
            Ordering::Equal => {
                root[y_res as usize] = x_res;
                rank[x_res as usize] += 1;
            }
            Ordering::Greater => root[y_res as usize] = x_res,
        }
    }
}

mod tests {
    use crate::path_exists_graph::Solution;

    #[test]
    fn test() {
        assert!(Solution::valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }
}
