```markdown
Given A range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.

Example 1:

Input: [5,7]
Output: 4

Example 2:

Input: [0,1]
Output: 0
```
```markdown
9   -- 0 0 0 0 1 0 0 1
10  -- 0 0 0 0 1 0 1 0
11  -- 0 0 0 0 1 0 1 1
12  -- 0 0 0 0 1 1 0 0
```

To solve this task for O(1) time we should iterate over binary repr of tne number n and m (given for example) and found common prefix.

```rust
struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut shift = 0;
        let mut mr = m;
        let mut nr = n;

        while mr < nr {
            mr >>= 1;
            nr >>= 1;
            shift += 1;
        }

        mr << shift
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::range_bitwise_and(9, 12), 8);
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    assert_eq!(Solution::range_bitwise_and(0, 2), 0);
}
```