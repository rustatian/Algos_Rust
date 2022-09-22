use std::collections::HashMap;

struct NonRepeating {}

impl NonRepeating {
    pub fn find_first_non_repeating_character(str: &str) -> isize {
        let mut hash: HashMap<u8, usize> = HashMap::new();

        for b in str.as_bytes() {
            if let Some(val) = hash.get_mut(b) {
                *val += 1;
            } else {
                hash.insert(*b, 1);
            }
        }

        for (idx, b) in str.as_bytes().iter().enumerate() {
            if *hash.get(b).unwrap() == 1 {
                return idx as isize;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::first_non_repeating::NonRepeating;

    #[test]
    fn test1() {
        assert_eq!(
            NonRepeating::find_first_non_repeating_character("abcdcaf"),
            1
        );
    }
}
