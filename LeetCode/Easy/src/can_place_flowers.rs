struct Solution {}

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut counter = 0;

        for idx in 0..flowerbed.len() {
            if flowerbed[idx] == 0 {
                let left: bool = (idx == 0) || flowerbed[idx - 1] == 0;
                let right: bool = (idx == flowerbed.len() - 1) || flowerbed[idx + 1] == 0;

                if left && right {
                    *flowerbed.get_mut(idx).unwrap() = 1;

                    counter += 1;
                    if counter >= n {
                        return true;
                    }
                }
            }
        }

        counter >= n
    }
}

mod tests {
    use crate::can_place_flowers::Solution;

    #[test]
    fn test() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
    }
}
