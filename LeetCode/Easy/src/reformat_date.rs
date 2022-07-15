struct Solution {}

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let date = date.split(' ').collect::<Vec<&str>>();
        let day = match &date[0][0..&date[0].len() - 2].len() {
            1 => {
                format!("0{}", &date[0][0..&date[0].len() - 2])
            }
            _ => date[0][0..&date[0].len() - 2].to_string(),
        };
        let month = match date[1] {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => panic!("error"),
        };

        let yr = date[2];
        format!("{}-{}-{}", yr, month, day)
    }
}

mod tests {
    use crate::reformat_date::Solution;

    #[test]
    fn test() {
        assert_eq!(
            "2052-10-20".to_string(),
            Solution::reformat_date("20th Oct 2052".to_string())
        )
    }
}
