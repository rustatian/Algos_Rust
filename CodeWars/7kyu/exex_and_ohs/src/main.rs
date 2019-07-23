// Check to see if a string has the same amount of 'x's and 'o's.
// The method must return a boolean and be case insensitive. The string can contain any char.
// XO("ooxx") => true
// XO("xooxx") => false
// XO("ooxXm") => true
// XO("zpzpzpp") => true // when no 'x' and 'o' is present should return true
// XO("zzoo") => false
fn main() {
    println!("Hello, world!");
}

fn xo(string: &'static str) -> bool {
    let lower = string.to_lowercase();

    let mut x_count: u8 = 0;
    let mut o_count: u8 = 0;

    for c in lower.chars() {
        match c {
            'x' => { x_count = x_count + 1; }
            'o' => { o_count = o_count + 1; }
            _ => {}
        }
    }

    if x_count == o_count {
        return true
    }
    return false
}

// best practise
fn xo2(string: &'static str) -> bool {
    string.chars().fold(0, |a, c|{
        match c {
            'x' | 'X' => a + 1,
            'o' | 'O' => a - 1,
            _ => a
        }
    }) == 0
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
