// Say you have an array for which the ith element is the price of a given stock on day i.
//
// If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.
//
// Note that you cannot sell a stock before you buy one.
//
// Example 1:
//
// Input: [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Not 7-1 = 6, as selling price needs to be larger than buying price.
// Example 2:
//
// Input: [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transaction is done, i.e. max profit = 0.


pub struct Solution {}


impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // for 0 and 1 elements vector we can't do the deal
        if prices.is_empty() || prices.len() == 1 {
            return 0;
        }

        // then we start iterate over the all N indexes of elements
        let mut idx = 1;
        // initial profit is zero
        let mut profit = 0;
        // random very big number
        let mut min_price = prices[0];
        loop {
            // standard loop
            if idx == prices.len() {
                break;
            }
            // we have to find valley and following by it maximum peak
            // here we actually set min_price to first point
            if prices[idx] < min_price {
                min_price = prices[idx];
            }

            // next we pull the maximum point which is connected to prev. minimum
            // if prices[idx] - min_price then prices[idx] is larger that prevv prices[idx] (new local function extremum)
            if prices[idx] - min_price > profit {
                // if true, we have a new profit
                profit = prices[idx] - min_price;
            }
            // count the index
            idx += 1;
        }

        profit
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
    assert_eq!(Solution::max_profit(vec![2, 4, 1]), 2);
}