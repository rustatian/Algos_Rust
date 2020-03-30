use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

//Task
//
//Given a list of digits, return the smallest number that could be formed from these digits, using the digits only once (ignore duplicates).
//Notes:
//
//Only positive integers will be passed to the function (> 0 ), no negatives or zeros.
//
//Input >> Output Examples
//
//minValue ({1, 3, 1})  ==> return (13)
//
//Explanation:
//
//(13) is the minimum number could be formed from {1, 3, 1} , Without duplications
//
//minValue({5, 7, 5, 9, 7})  ==> return (579)
//
//Explanation:
//
//(579) is the minimum number could be formed from {5, 7, 5, 9, 7} , Without duplications
//
//minValue({1, 9, 3, 1, 7, 4, 6, 6, 7}) return  ==> (134679)
//
//Explanation:
//
//(134679) is the minimum number could be formed from {1, 9, 3, 1, 7, 4, 6, 6, 7} , Without duplications

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();

    digits.into_iter().fold(0, |acc, x| acc * 10 + x)
}

// Used to convert any iterator into a sorted vec.
fn harness(values: impl IntoIterator<Item = i32>) -> Vec<i32> {
    let mut values: Vec<_> = values.into_iter().collect();
    values.sort();
    values
}


#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}


























