//Given a valid (IPv4) IP address, return a defanged version of that IP address.
//
//A defanged IP address replaces every period "." with "[.]".
//
//Example 1:
//
//Input: address = "1.1.1.1"
//Output: "1[.]1[.]1[.]1"
//
//Example 2:
//
//Input: address = "255.100.50.0"
//Output: "255[.]100[.]50[.]0"

pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

fn testing(address: String, res: String) -> () {
    assert_eq!(defang_i_paddr(address), res)
}

#[test]
fn test_def_str() {
    testing("255.100.50.0".to_string(), "255[.]100[.]50[.]0".to_string());
}
