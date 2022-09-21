struct Solution {}

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut root = vec![0; n as usize];
        let mut rank = vec![0; n as usize];

        for i in 0..n {
            root[i as usize] = i;
            rank[i as usize] = 1;
        }

        let mut count = n;

        for edge in &edges {
            count -= Solution::union(edge[0], edge[1], &mut root, &mut rank);
        }

        count
    }

    pub fn find(x: i32, root: &mut Vec<i32>) -> i32 {
        if x == root[x as usize] {
            return x;
        }

        root[x as usize] = Solution::find(root[x as usize], root);

        root[x as usize]
    }
    pub fn union(x: i32, y: i32, root: &mut Vec<i32>, rank: &mut Vec<i32>) -> i32 {
        let root_x = Solution::find(x, root);
        let root_y = Solution::find(y, root);

        if root_x == root_y {
            return 0;
        }

        use std::cmp::Ordering;
        use std::ops::Add;

        match rank[root_x as usize].cmp(&rank[root_y as usize]) {
            Ordering::Less => root[root_x as usize] = root_y,
            Ordering::Equal => {
                root[root_y as usize] = root_x;
                let _ = rank.get_mut(root_x as usize).unwrap().add(1);
            }
            Ordering::Greater => root[root_y as usize] = root_x,
        }

        1
    }
}

mod tests {
    #[test]
    fn test() {}
}
