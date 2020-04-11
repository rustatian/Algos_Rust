//Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).
//
//Examples:
//
//solution("abc", "bc") //returns true
//solution("abc", "d") //returns false

fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}


// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));
}