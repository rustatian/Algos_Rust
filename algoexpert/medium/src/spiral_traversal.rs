struct SpiralTraverse {}

impl SpiralTraverse {
    fn traverse(array: &[&[u8]]) -> Vec<u8> {
        let mut res = vec![];

        let mut start_row = 0;
        let mut end_row = array.len() - 1;
        let mut start_column = 0;
        let mut end_column = array[0].len() - 1;

        while start_row <= end_row && start_column <= end_column {
            for i in start_column..=end_column {
                res.push(array[start_row][i]);
            }

            for row in (start_row + 1)..=end_row {
                res.push(array[row][end_column]);
            }

            for i in (start_column..=(end_column - 1)).rev() {
                if start_row == end_row {
                    break;
                }
                res.push(array[end_row][i]);
            }

            for row in (start_row + 1..=(end_row - 1)).rev() {
                if start_column == end_column {
                    break;
                }
                res.push(array[row][start_column]);
            }

            start_column += 1;
            end_column -= 1;
            start_row += 1;
            end_row -= 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::spiral_traversal::SpiralTraverse;

    #[test]
    fn test() {
        let arr: &[&[u8]] = &[&[1, 2, 3, 4], &[10, 11, 12, 5], &[9, 8, 7, 6]];

        assert_eq!(
            SpiralTraverse::traverse(&arr),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        );
    }
}
