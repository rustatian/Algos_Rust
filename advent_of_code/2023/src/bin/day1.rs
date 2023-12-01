use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("advent_of_code/2023/src/bin/data/input_1.txt").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let splitted: Vec<_> = s.split('\n').collect();
    let mut res = 0;

    for v in &splitted {
        let mut n1 = 0;
        let mut n2 = 9999;
        let mut first = true;
        for c in v.chars() {
            if c.is_ascii_digit() {
                if first {
                    n1 = c.to_digit(10).unwrap();
                    first = false;
                    continue;
                } else {
                    n2 = c.to_digit(10).unwrap();
                }
            }
        }

        res += n1 * 10 + if n2 == 9999 { n1 } else { n2 };
    }

    res = 0;
    for v in &splitted {
        let mut n1 = 0;
        let mut n2 = 9999;
        let mut first = true;

        let chars = v.chars().collect::<Vec<_>>();
        let mut idx = 0;
        while idx < chars.len() {
            if chars[idx].is_ascii_digit() {
                if first {
                    n1 = chars[idx].to_digit(10).unwrap();
                    first = false;
                } else {
                    n2 = chars[idx].to_digit(10).unwrap();
                }

                idx += 1;
                continue;
            }

            // one, two, three, four, five, six, seven, eight, and nine
            match chars[idx] {
                'o' => {
                    // one
                    if idx + 2 < chars.len() && chars[idx + 1] == 'n' && chars[idx + 2] == 'e' {
                        if first {
                            n1 = 1;
                            first = false;
                            idx += 1;
                        } else {
                            n2 = 1;
                            idx += 1;
                        }
                    } else {
                        idx += 1;
                    }
                }
                't' => {
                    // two
                    if idx + 2 < chars.len() && chars[idx + 1] == 'w' && chars[idx + 2] == 'o' {
                        if first {
                            n1 = 2;
                            first = false;
                            idx += 1;
                            continue;
                        } else {
                            n2 = 2;
                            idx += 1;
                            continue;
                        }
                    }
                    // three
                    if idx + 4 < chars.len()
                        && chars[idx + 1] == 'h'
                        && chars[idx + 2] == 'r'
                        && chars[idx + 3] == 'e'
                        && chars[idx + 4] == 'e'
                    {
                        if first {
                            n1 = 3;
                            first = false;
                            idx += 1;
                            continue;
                        } else {
                            n2 = 3;
                            idx += 1;
                            continue;
                        }
                    }

                    idx += 1;
                }
                'f' => {
                    // four
                    if idx + 3 < chars.len()
                        && chars[idx + 1] == 'o'
                        && chars[idx + 2] == 'u'
                        && chars[idx + 3] == 'r'
                    {
                        if first {
                            n1 = 4;
                            first = false;
                            idx += 1;
                            continue;
                        } else {
                            n2 = 4;
                            idx += 1;
                            continue;
                        }
                    }
                    // five
                    if idx + 3 < chars.len()
                        && chars[idx + 1] == 'i'
                        && chars[idx + 2] == 'v'
                        && chars[idx + 3] == 'e'
                    {
                        if first {
                            n1 = 5;
                            first = false;
                            idx += 1;
                            continue;
                        } else {
                            n2 = 5;
                            idx += 1;
                            continue;
                        }
                    }

                    idx += 1;
                }
                's' => {
                    // six
                    if idx + 2 < chars.len() && chars[idx + 1] == 'i' && chars[idx + 2] == 'x' {
                        if first {
                            n1 = 6;
                            first = false;
                            idx += 1;
                            continue;
                        } else {
                            n2 = 6;
                            idx += 1;
                            continue;
                        }
                    }

                    // seven
                    if idx + 4 < chars.len()
                        && chars[idx + 1] == 'e'
                        && chars[idx + 2] == 'v'
                        && chars[idx + 3] == 'e'
                        && chars[idx + 4] == 'n'
                    {
                        if first {
                            n1 = 7;
                            first = false;
                            idx += 1;
                            continue;
                        } else {
                            n2 = 7;
                            idx += 1;
                            continue;
                        }
                    }

                    idx += 1;
                }
                'e' => {
                    // eight
                    if idx + 4 < chars.len()
                        && chars[idx + 1] == 'i'
                        && chars[idx + 2] == 'g'
                        && chars[idx + 3] == 'h'
                        && chars[idx + 4] == 't'
                    {
                        if first {
                            n1 = 8;
                            first = false;
                            idx += 1;
                        } else {
                            n2 = 8;
                            idx += 1;
                        }
                    } else {
                        idx += 1;
                    }
                }
                'n' => {
                    // nine
                    if idx + 3 < chars.len()
                        && chars[idx + 1] == 'i'
                        && chars[idx + 2] == 'n'
                        && chars[idx + 3] == 'e'
                    {
                        if first {
                            n1 = 9;
                            first = false;
                            idx += 1;
                        } else {
                            n2 = 9;
                            idx += 1;
                        }
                    } else {
                        idx += 1;
                    }
                }
                _ => {
                    idx += 1;
                }
            }
        }

        res += n1 * 10 + if n2 == 9999 { n1 } else { n2 };
    }

    println!("num: {}", res);
}
