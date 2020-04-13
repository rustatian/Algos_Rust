// Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.
//
// Example 1:
//
// Input: [0,1]
// Output: 2
// Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.
//
// Example 2:
//
// Input: [0,1,0]
// Output: 2
// Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
//
// Note: The length of the given binary array will not exceed 50,000.

// Method 2: This is an efficient approach which uses Hashmap to arrive at the solution.
//
// Approach: The concept of taking cumulative sum, taking 0’s as -1 will help us in optimising the approach.
// While taking the cumulative sum, there are two cases when there can be a sub-array with equal number of 0’s and 1’s.
// One, when cumulative sum=0, which signifies that sub-array from index (0) till present index has equal number of 0’s and 1’s.
// Two, when we encounter a cumulative sum value which we have already encountered before,
// which means that sub-array from the previous index+1 till the present index has equal number of 0’s and 1’s as they give a cumulative sum of 0 .
//

// Let input array be arr[] of size n and max_size be the size of output sub-array.
// Create a temporary array sumleft[] of size n. Store the sum of all elements from arr[0] to arr[i] in sumleft[i].
// There are two cases, the output sub-array may start from 0th index or may start from some other index. We will return the max of the values obtained by two cases.
// To find the maximum length sub-array starting from 0th index, scan the sumleft[] and find the maximum i where sumleft[i] = 0.
// Now, we need to find the subarray where subarray sum is 0 and start index is not 0.
// This problem is equivalent to finding two indexes i & j in sumleft[] such that sumleft[i] = sumleft[j] and j-i is maximum.
// To solve this, we create a hash table with size = max-min+1 where min is the minimum value in the sumleft[] and max is the maximum value in the sumleft[].
// Hash the leftmost occurrences of all different values in sumleft[].
// The size of hash is chosen as max-min+1 because there can be these many different possible values in sumleft[]. Initialize all values in hash as -1.
// To fill and use hash[], traverse sumleft[] from 0 to n-1.
// If a value is not present in hash[], then store its index in hash.
// If the value is present, then calculate the difference of current index of sumleft[] and previously stored value in hash[].
// If this difference is more than maxsize, then update the maxsize.
// To handle corner cases (all 1s and all 0s), we initialize maxsize as -1. If the maxsize remains -1, then print there is no such subarray.

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let nums_len = nums.len();

        let mut max_idx = 0;
        let mut arr: Vec<i32> = Vec::with_capacity(nums.len());
        let mut hash: HashMap<i32, i32> = HashMap::new();


        for num in &nums {
            if *num == 0 {
                arr.push(-1)
            } else {
                arr.push(1)
            }
        }

        let mut idx: i32 = 0;
        let mut sum = 0;
        while idx < nums_len as i32 {
            sum += arr[idx as usize];
            if sum == 0 {
                max_idx = idx + 1;
            }
            if hash.contains_key(&(sum + nums_len as i32)) {
                if max_idx < idx - hash.get(&(sum + nums_len as i32)).unwrap() {
                    max_idx = idx - hash.get(&(sum + nums_len as i32)).unwrap();
                }
            } else {
                hash.insert(sum + nums_len as i32, idx);
            }
            idx += 1;
        }

        max_idx
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]), 6);
}