// We have a collection of stones, each stone has a positive integer weight.
//
// Each turn, we choose the two heaviest stones and smash them together.  Suppose the stones have weights x and y with x <= y.  The result of this smash is:
//
// If x == y, both stones are totally destroyed;
// If x != y, the stone of weight x is totally destroyed, and the stone of weight y has new weight y-x.
//
// At the end, there is at most 1 stone left.  Return the weight of this stone (or 0 if there are no stones left.)
//
//
//
// Example 1:
//
// Input: [2,7,4,1,8,1]
// Output: 1
// Explanation:
// We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
// we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
// we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
// we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of last stone.
//
//
//
// Note:
//
// 1 <= stones.length <= 30
// 1 <= stones[i] <= 1000
//

struct Solution {}

// impl Solution {
//     pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
//         if stones.len() == 1 {
//             return stones[0];
//         }
//         if stones.len() == 2 {
//             return (stones[0] - stones[1]).abs();
//         }
//
//         Solution::recursion(stones)
//     }
//
//     fn recursion(stones: Vec<i32>) -> i32 {
//         if stones.len() == 1 {
//             return stones[0];
//         }
//         let mut stones_copy = stones;
//         stones_copy.sort_by(|a, b| b.cmp(a));
//
//
//         let x = stones_copy[0];
//         let y = stones_copy[1];
//
//         stones_copy.remove(0);
//         stones_copy.remove(0);
//         stones_copy.push(x - y);
//
//
//         Solution::recursion(stones_copy)
//     }
// }

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(stones);

        while heap.len() > 1 {
            if let Some(stone1) = heap.pop() {
                if let Some(stone2) = heap.pop() {
                    if stone1 > stone2 {
                        let res = stone1 - stone2;
                        heap.push(res);
                    }
                }
            }
        }

        if heap.is_empty() {
            return 0;
        }

        heap.pop().unwrap()
    }
}


#[test]
fn tests() {
    assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(Solution::last_stone_weight(vec![3, 7, 2]), 2);
    assert_eq!(Solution::last_stone_weight(vec![3, 7, 8]), 2);
    assert_eq!(Solution::last_stone_weight(vec![7, 6, 7, 6, 9]), 3);
    assert_eq!(Solution::last_stone_weight(vec![2, 4, 1, 6, 10, 2, 1, 7, 9]), 0);
    assert_eq!(Solution::last_stone_weight(vec![5, 1, 8, 10, 7]), 1);
    assert_eq!(Solution::last_stone_weight(vec![10, 4, 2, 10]), 2);
    assert_eq!(Solution::last_stone_weight(vec![316, 157, 73, 106, 771, 828, 46, 212, 926, 604, 600, 992, 71, 51, 477, 869, 425, 405, 859, 924, 45, 187, 283, 590, 303, 66, 508, 982, 464, 398]), 0);
}