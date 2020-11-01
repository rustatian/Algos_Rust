struct MoveElementToEnd {}

impl MoveElementToEnd {
    fn run(v: Vec<i32>, to_move: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = v.len() - 1;
        let mut target = v;

        while left <= right {
            if target[left] != to_move {
                left += 1;
                continue;
            }
            if target[right] == to_move {
                right -= 1;
                continue;
            }

            target.swap(right, left);
            right -= 1;
            left += 1;
        }

        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(MoveElementToEnd::run(vec![5, 1, 2, 5, 5, 3, 4, 6, 7, 5, 8, 9, 10, 11, 5, 5, 12], 5),
                   vec![12, 1, 2, 11, 10, 3, 4, 6, 7, 9, 8, 5, 5, 5, 5, 5, 5])
    }
}