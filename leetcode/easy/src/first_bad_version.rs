struct Solution{}

impl Solution {
    pub fn isBadVersion(&self, _:i32) -> bool {
       true
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut start= 1;
        let mut end = n;

        while end != start {
            let middle = start + (end - start) / 2;
            if self.isBadVersion(middle) {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

       end
    }
}

