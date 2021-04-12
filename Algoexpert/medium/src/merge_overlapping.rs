use std::cmp::max;
use std::ops::Index;

struct MergeOverlapping {}

impl MergeOverlapping {
    fn merge_overlapping(intervals: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut intervals_clone = intervals;
        intervals_clone.sort();

        let mut curr = intervals_clone[0].clone();
        let mut merged = vec![curr.clone()];

        for (_, interval) in intervals_clone.iter().enumerate() {
            let curr_end = curr[1];
            let next_start = interval[0];
            let next_end = interval[1];

            if curr_end >= next_start {
                let _ = std::mem::replace(&mut curr[1], max(curr_end, next_end));
            } else {
                println!("before: {:?}", curr);
                curr = interval.clone();
                println!("after: {:?}", curr);
                merged.push(curr.clone());
            }
        }

        merged
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_overlapping::MergeOverlapping;

    #[test]
    fn test() {
        let arr = vec![vec![1, 2], vec![3, 5], vec![6, 8], vec![9, 10], vec![4, 7]];
        let res = vec![vec![1, 2], vec![3, 8], vec![9, 10]];
        assert_eq!(MergeOverlapping::merge_overlapping(arr), res);
    }
}
