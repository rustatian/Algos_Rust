struct PalindromeCheck {}

impl PalindromeCheck {
    fn check(v: &str) -> bool {
        let bytes = v.as_bytes();
        let mut i = 0;
        let mut j = bytes.len() - 1;

        while i <= j {
            if bytes[i] == bytes[j] {
                i += 1;
                j -= 1;
                continue;
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::palindrome_check::PalindromeCheck;

    #[test]
    fn test() {
        assert_eq!(PalindromeCheck::check("abcdcba"), true);
    }
}
