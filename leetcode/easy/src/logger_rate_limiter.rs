use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Logger {
    map: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 *
**/

impl Logger {
    fn new() -> Self {
        Logger {
            map: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        return match self.map.entry(message) {
            Entry::Occupied(mut ts) => {
                // 12 >= 11
                if timestamp >= *ts.get() {
                    let val = ts.get_mut();
                    *val = timestamp + 10;
                    return true;
                }

                false
            }
            Entry::Vacant(en) => {
                en.insert(timestamp + 10);
                true
            }
        };
    }
}

mod tests {
    use crate::logger_rate_limiter::Logger;

    #[test]
    fn test() {
        let mut l = Logger::new();
        assert!(l.should_print_message(0, "A1".to_string()));
        assert!(l.should_print_message(3, "A4".to_string()));
        assert!(l.should_print_message(6, "A0".to_string())); // 16
        assert!(l.should_print_message(9, "A3".to_string()));
        assert!(!l.should_print_message(12, "A3".to_string()));
        assert!(l.should_print_message(15, "A4".to_string()));
        assert!(!l.should_print_message(18, "A3".to_string()));
        assert!(l.should_print_message(21, "A2".to_string()));
        assert!(l.should_print_message(24, "A1".to_string()));
        assert!(!l.should_print_message(27, "A2".to_string()));
        assert!(l.should_print_message(30, "A0".to_string())); // 30 + 10
        assert!(!l.should_print_message(33, "A0".to_string()));
        assert!(l.should_print_message(36, "A4".to_string()));

    }
}