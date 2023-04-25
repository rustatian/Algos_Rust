/**
 * Forward declaration of guess API
 * @param  num   your gues
 * @return 	     -1 if num is higher
 *			      1 if num is lower
 *               otherwise return
 * unsafe fn guess(num: i32) -> i32 {
 */

struct Solution {}

fn guess(num: i32) -> i32 {
    if num > 1150769282 {
        return -1;
    }

    if num < 1150769282 {
        return 1;
    }

    0
}

impl Solution {
    fn guess_number(n: i32) -> i32 {
        if guess(n) == 0 {
            return n;
        }

        let mut low = 1;
        let mut high = n;

        while low <= high {
            let middle = low + (high - low) / 2;
            let res = guess(middle);

            match res {
                0 => {
                    return middle;
                }
                -1 => {
                    high = middle - 1;
                }
                1 => {
                    low = middle + 1;
                }

                _ => {
                    panic!("foo")
                }
            }
        }
        -1
    }
}

mod tests {
    use crate::guess_number::Solution;

    #[test]
    fn test() {
        assert_eq!(1150769282, Solution::guess_number(1420736637));
    }
}
