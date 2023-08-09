struct Solution{}

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
	nums.sort_unstable();

	let mut l = 0;
	let mut r = nums[nums.len()-1]- nums[0];

	while l < r {
		let middle = l + (r - l)/2;
		if Solution::helper(&nums, middle) >= p {
			r = middle;
		} else {
			l = middle + 1;
		}
	}

	l
    }

    pub fn helper(nums: &Vec<i32>, p:i32) -> i32 {
	let mut idx = 0;
	let mut num = 0;

	while idx < nums.len() - 1 {
	    if nums[idx + 1] - nums[idx] <= p {
		num += 1;
		idx += 1;
	    }
	    idx += 1;
	}
	num
    }
}

#[cfg(test)]
mod tests{
	#[test]
	fn test(){}
}