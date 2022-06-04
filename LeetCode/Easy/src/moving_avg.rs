/*
Given a stream of integers and a window size, calculate the moving average of all integers in the sliding window.

Implement the MovingAverage class:

MovingAverage(int size) Initializes the object with the size of the window size.
double next(int val) Returns the moving average of the last size values of the stream.


Example 1:

Input
["MovingAverage", "next", "next", "next", "next"]
[[3], [1], [10], [3], [5]]
Output
[null, 1.0, 5.5, 4.66667, 6.0]

Explanation
MovingAverage movingAverage = new MovingAverage(3);
movingAverage.next(1); // return 1.0 = 1 / 1
movingAverage.next(10); // return 5.5 = (1 + 10) / 2
movingAverage.next(3); // return 4.66667 = (1 + 10 + 3) / 3
movingAverage.next(5); // return 6.0 = (10 + 3 + 5) / 3
*/

struct MovingAverage {
    window: i32,
    data: Vec<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            window: size,
            data: vec![],
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.data.push(val);

        let divider: i32 = {
            if self.data.len() < self.window as usize {
                self.data.len() as i32
            } else {
                self.window
            }
        };

        let sum: i32 = self.data.iter().rev().take(self.window as usize).sum();

        sum as f64/ divider as f64
    }
}

mod tests {
    use crate::moving_avg::MovingAverage;

    #[test]
    fn test() {
        let mut obj = MovingAverage::new(3);
        let ret_1: f64 = obj.next(1);
        assert_eq!(ret_1, 1.0);

        let ret_2: f64 = obj.next(10);
        assert_eq!(ret_2, 5.5);

        let ret_3: f64 = obj.next(3);
        assert_eq!(ret_3, 4.666666666666667);

        let ret_4: f64 = obj.next(5);
        assert_eq!(ret_4, 6.0);
    }
}
