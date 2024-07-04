struct Solution {}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        if pairs.is_empty() {
            return s;
        }
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;
        let mut s = s.chars().collect::<Vec<char>>();

        let mut root = vec![0; s.len()];
        let mut rank = vec![0; s.len()];

        for i in 0..s.len() {
            root[i] = i as i32;
            rank[i] = 1;
        }

        for pair in &pairs {
            Solution::union(pair[0], pair[1], &mut root, &mut rank);
        }

        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..root.len() {
            let key = Solution::find(root[i], &mut root);
            match hm.entry(key) {
                Entry::Occupied(mut oe) => oe.get_mut().push(i as i32),
                Entry::Vacant(_) => {
                    hm.insert(key, vec![i as i32]);
                }
            }
        }

        // all letters connected to each other
        if hm.len() == 1 {
            s.sort_unstable();
            return s.into_iter().collect::<String>();
        }

        let mut ret: Vec<char> = vec![' '; s.len()];

        for v in hm.values() {
            if v.len() == 1 {
                ret[v[0] as usize] = s[v[0] as usize];
                continue;
            }

            let mut str = vec![];

            for vals in v {
                str.push(s[*vals as usize]);
            }

            str.sort_unstable();

            for (idx, ch) in v.iter().enumerate() {
                ret[*ch as usize] = str[idx];
            }
        }

        ret.into_iter().collect::<String>()
    }

    pub fn find(x: i32, root: &mut [i32]) -> i32 {
        if x == root[x as usize] {
            return x;
        }

        root[x as usize] = Solution::find(root[x as usize], root);

        root[x as usize]
    }

    pub fn union(x: i32, y: i32, root: &mut [i32], rank: &mut [i32]) {
        let root_x = Solution::find(x, root);
        let root_y = Solution::find(y, root);

        if root_x == root_y {
            return;
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
    }
}

mod tests {
    use crate::smallest_string_with_swaps::Solution;

    #[test]
    fn test() {
        assert_eq!(
            String::from("tfikklmqxlyz"),
            Solution::smallest_string_with_swaps(
                String::from("tklkxyizmlqf"),
                vec![
                    vec![2, 10],
                    vec![3, 5],
                    vec![8, 11],
                    vec![1, 2],
                    vec![10, 6],
                    vec![4, 1],
                    vec![1, 10],
                    vec![5, 8],
                    vec![8, 3],
                    vec![10, 4],
                    vec![7, 3],
                    vec![10, 11]
                ]
            )
        );
        // assert_eq!(
        //     String::from("deykuy"),
        //     Solution::smallest_string_with_swaps(
        //         String::from("udyyek"),
        //         vec![
        //             vec![3, 3],
        //             vec![3, 0],
        //             vec![5, 1],
        //             vec![3, 1],
        //             vec![3, 4],
        //             vec![3, 5]
        //         ]
        //     )
        // );
        // assert_eq!(
        //     String::from("bacd"),
        //     Solution::smallest_string_with_swaps(
        //         String::from("dcab"),
        //         vec![vec![0, 3], vec![1, 2]]
        //     )
        // );
    }
}
