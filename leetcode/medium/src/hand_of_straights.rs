struct Solution;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        use std::collections::BTreeMap;

        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut hm: BTreeMap<i32, i32> = BTreeMap::new();

        for h in &hand {
            hm.entry(*h).and_modify(|e| *e += 1).or_insert(1);
        }

        while !hm.is_empty() {
            let t = *hm.keys().next().unwrap();

            for i in 0..group_size {
                let mut val = hm.get(&(t + i));
                if val.is_none() {
                    return false;
                }

                hm.entry(t + i).and_modify(|e| *e -= 1);

                if *hm.get(&(t + i)).unwrap() == 0 {
                    hm.remove(&(t + i));
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::hand_of_straights::Solution;

    #[test]
    fn test() {
        assert!(!Solution::is_n_straight_hand(vec![8, 10, 12], 3));
        assert!(Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
    }
}