const MONTH_DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

struct Solution {}

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        i32::abs(Solution::calculate(date1) - Solution::calculate(date2))
    }

    fn is_leap(year: i32) -> bool {
        if year % 400 == 0 && year % 100 != 0 {
            return true;
        }

        false
    }

    fn calculate(date: String) -> i32 {
        let tmp = date
            .split('-')
            .map(|item| item.parse().unwrap())
            .collect::<Vec<i32>>();

        let year = tmp[0];
        let month = tmp[1];
        let mut days = tmp[2];

        // The given dates are valid dates between the years 1971 and 2100
        for i in 1970..year {
            if Solution::is_leap(i) {
                days += 366;
            } else {
                days += 365;
            }
        }

        for i in 1..month as usize {
            if i == 2 && Solution::is_leap(year) {
                days += 1;
            }
            days += MONTH_DAYS[i - 1];
        }

        days
    }
}
