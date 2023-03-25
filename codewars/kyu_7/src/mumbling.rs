//accum("abcd") -> "A-Bb-Ccc-Dddd"
//accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
//accum("cwAt") -> "C-Ww-Aaa-Tttt"

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_tests() {
    assert_eq!(accum_functional("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
    assert_eq!(accum_functional("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
    assert_eq!(accum_functional("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
    assert_eq!(accum_functional("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
    assert_eq!(accum_functional("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}

// my first approach
fn accum(s: &str) -> String {
    let mut str_new = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            str_new.push(c);
            for j in 0..i {
                str_new.push(c.to_ascii_lowercase());
            }
            str_new.push('-');
            continue;
        }

        str_new.push(c.to_ascii_uppercase());
        for j in 0..i {
            str_new.push(c);
        }
        str_new.push('-');
    }

    let _ = str_new.pop();


    str_new
}

// functional solution
fn accum_functional(s: &str) -> String {
    s.chars().enumerate().map(|(i, c)| c.to_string().to_uppercase() +
            &(0..i).map(|_| c.to_string().to_lowercase()).collect::<String>())
        .collect::<Vec<_>>().join("-")
}