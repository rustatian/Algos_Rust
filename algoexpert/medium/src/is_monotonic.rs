struct IsMonotonic {}

impl IsMonotonic {
    fn is_monotonic(v: Vec<i32>) -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                increasing = false;
                continue;
            }
            if v[i] < v[i + 1] {
                decreasing = false;
                continue;
            }
        }

        increasing || decreasing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            IsMonotonic::is_monotonic(vec![-1, -5, -10, -1100, -900, -1101, -1102, -9001]),
            false
        );
    }
}
