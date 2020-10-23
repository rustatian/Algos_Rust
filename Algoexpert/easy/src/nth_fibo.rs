struct NthFibo {}

impl NthFibo {
    fn get_nth_fibo(n: i32) -> i32 {
        match n {
            0..=1 => 0,
            2 => 1,
            _ => {
                let mut fibo = 1;
                let mut prev_fibo = 1;
                let mut i = 2;

                while i < n {
                    let tmp = fibo;
                    fibo += prev_fibo;
                    prev_fibo = tmp;
                    i += 1;
                }

                prev_fibo
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nth_fibo::NthFibo;

    #[test]
    fn test() {
        let n = NthFibo::get_nth_fibo(6);
        assert_eq!(n, 5);
    }
}
