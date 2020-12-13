struct ArrayOfProducts {}

impl ArrayOfProducts {
    fn solve(data: Vec<isize>) -> Vec<isize> {
        let mut res = Vec::with_capacity(data.len());
        let mut left = Vec::with_capacity(data.len());
        let mut right = vec![0; data.len()];

        let mut i = 1;
        let mut product = data[0];
        left.push(1);

        while i < data.len() {
            left.push(product);
            product *= data[i];
            i += 1
        }

        let mut i = data.len() - 2;
        let mut product = data[data.len() - 1];
        right[data.len() - 1] = 1;

        while i > 0 {
            right[i] = product;
            product *= data[i];
            i -= 1
        }
        right[0] = product;

        for _ in 0..data.len() {
            res.push(left[i] * right[i]);
            i += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::array_of_products::ArrayOfProducts;

    #[test]
    fn test() {
        let expected = vec![8, 40, 10, 20];
        assert_eq!(ArrayOfProducts::solve(vec![5, 1, 4, 2]), expected)
    }
}
