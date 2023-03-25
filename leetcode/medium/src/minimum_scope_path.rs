struct Solution {}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut root = vec![0; (n + 1) as usize];

        for i in 1..n + 1 {
            root[i as usize] = i;
        }

        for edge in &roads {
            Solution::union(edge[0], edge[1], &mut root);
        }

        let mut res = 100000;

        for edge in &roads {
            if Solution::find(1, &mut root) == Solution::find(edge[0], &mut root) {
                res = std::cmp::min(res, edge[2])
            }
        }

        return res;
    }

    pub fn find(x: i32, root: &mut Vec<i32>) -> i32 {
        if x == root[x as usize] {
            return x;
        }

        root[x as usize] = Solution::find(root[x as usize], root);

        root[x as usize]
    }
    pub fn union(x: i32, y: i32, root: &mut Vec<i32>) {
        let root_x = Solution::find(x, root);
        let root_y = Solution::find(y, root);

        if root_x == root_y {
            return;
        }

        root[root_y as usize] = root_x;
    }
}

mod tests {
    use crate::minimum_scope_path::Solution;

    #[test]
    fn test() {
        Solution::min_score(
            418,
            vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]],
        );
    }
}
