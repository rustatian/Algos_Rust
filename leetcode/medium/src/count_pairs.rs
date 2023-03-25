

struct Solution {}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        use std::collections::hash_map::Entry;
        use std::collections::{HashMap};

        let mut root = vec![0; n as usize];

        for i in 0..n {
            root[i as usize] = i;
        }

        let mut hm: HashMap<i32, i32> = HashMap::new();

        for edge in &edges {
            Solution::union(edge[0], edge[1], &mut root);
        }

        let mut r_copy = root.clone();

        for i in root {
            let ans = Solution::find(i, &mut r_copy);

            match hm.entry(ans) {
                Entry::Occupied(mut entry) => {
                    (*entry.get_mut()) += 1;
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(1);
                }
            }
        }

        let mut answ = 0;
        let mut rest = n as usize;

        for (_, v) in hm {
            answ += v as usize * (rest - v as usize);
            rest -= v as usize;
        }

        answ as i64
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
    use crate::count_pairs::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            0,
            Solution::count_pairs(
                5,
                vec![vec![1, 0], vec![3, 1], vec![0, 4], vec![2, 1]]
            )
        )
    }
}
