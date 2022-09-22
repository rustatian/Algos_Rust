struct MinWaitTime {}

impl MinWaitTime {
    pub fn min_wait_queue(queries: &[u8]) -> u8 {
        let mut qcopy = queries.to_vec();
        qcopy.sort_unstable();

        let mut res = 0;

        let _ = qcopy.iter().skip(1).enumerate().fold(0, |mut acc, tuple| {
            acc += qcopy[tuple.0];
            res += acc;
            acc
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_wait_time::MinWaitTime;
    use std::borrow::Borrow;

    #[test]
    fn test() {
        assert_eq!(MinWaitTime::min_wait_queue([3, 2, 1, 2, 6].borrow()), 17);
    }
}
