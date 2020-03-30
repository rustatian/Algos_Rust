pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    s.chars().filter(|&stoun| j.contains(stoun)).count() as i32
}


fn testing(j: String, s: String) -> () {
    assert_eq!(num_jewels_in_stones(j, s), 3)
}

#[test]
fn test_def_str() {
    testing("aA".to_string(), "aAAbbbb".to_string());
}