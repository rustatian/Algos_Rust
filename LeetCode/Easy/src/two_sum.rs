use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut data = HashMap::new();

        for (k, v) in nums.iter().enumerate() {
            let diff = target - v;

            if data.contains_key(&diff) {
                return Vec::from([*data.get(&diff).unwrap() as i32, (k as i32)]);
            }

            data.insert(v, k);
        }

        vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::two_sum::Solution;

    #[test]
    fn test() {
        let vector = vec![2,7,11,15];

        let res = Solution::two_sum(vector, 9);
        println!("{:?}", res);
    }
}