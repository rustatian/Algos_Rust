struct LongestPeak {}

impl LongestPeak {
    fn solve(data: Vec<isize>) -> usize {
        let mut longest = 0;
        let mut i = 1;

        while i < data.len() - 1 {
            let peak = data[i - 1] < data[i] && data[i] > data[i + 1];
            if !peak {
                i += 1;
                continue;
            }

            let mut left = i - 2;
            while data[left] < data[left + 1] {
                left -= 1;
            }

            let mut right = i + 2;
            while right < data.len() && data[right] < data[right - 1] {
                right += 1;
            }

            let curr_peak = right - left - 1;
            if curr_peak > longest {
                longest = curr_peak;
            }
            i = right;
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_peak::LongestPeak;
    #[test]
    fn test() {
        let data = vec![1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
        assert_eq!(LongestPeak::solve(data), 6);
    }
}
