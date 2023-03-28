struct Solution {}

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut tmp: Vec<Vec<i32>> = vec![];

        for _ in 0..edges.len() {
            tmp.push(vec![0; 2])
        }

        for i in 0..edges.len() {
            let mut first = 1;
            let current: i32 = i as i32;

            while current != -1 && tmp[current as usize][1] == 0 {
                first += 1;
                tmp[current as usize] = vec![i as i32, first as i32]
            }

            if current != -1 && tmp[current as usize][0] == i as i32 {
                res = std::cmp::max(res, first - tmp[current as usize][1])
            }
        }

        println!("foo");

        return res as i32;
    }
}

mod tests {
    use crate::longest_cycle_in_a_graph::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
    }
}
