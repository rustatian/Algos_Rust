struct Solution {}

impl Solution {
    pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut logs = logs;
        logs.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut root = vec![0; n as usize];
        let mut rank = vec![0; n as usize];

        for i in 0..n {
            root[i as usize] = i;
            rank[i as usize] = 1;
        }

        let mut count = n;

        for (idx, log) in logs.iter().enumerate() {
            count -= Solution::union(log[1], log[2], &mut root, &mut rank);
            if count == 1 {
                return logs[idx][0];
            }
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
