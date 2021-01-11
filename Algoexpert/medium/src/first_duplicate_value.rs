use std::collections::HashMap;
pub struct FDV {}

impl FDV {
    pub fn solve(data: &mut [i32]) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for i in data {
            if hm.contains_key(&i) {
                return *i;
            }

            hm.insert(*i, *i);
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::first_duplicate_value::FDV;

    #[test]
    fn test() {
        let mut data = [2, 1, 5, 2, 3, 3, 4];
        assert_eq!(FDV::solve(&mut data), 2);
    }
}
