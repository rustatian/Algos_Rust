fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Omnibool {}

impl PartialEq<bool> for Omnibool {
    fn eq(&self, _other: &bool) -> bool {
        true
    }
}

#[allow(non_upper_case_globals)]
const omnibool: Omnibool = Omnibool {};


#[test]
fn equals_to_both() {
    assert!(omnibool == true && omnibool == false, "`OMNIBOOL` should be equal to both")
}