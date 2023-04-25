use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

struct SmallestInfiniteSet {
    queue: BinaryHeap<Reverse<i32>>,
    contains: HashSet<i32>,
    min: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self{
            contains: HashSet::new(),
            queue: BinaryHeap::new(),
            min: 1,
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        // check if the heap contains some elemets
        if self.queue.is_empty() {
            if self.min == 1 {
                // increment
                self.min += 1;
                return 1;
            } else {
                let mut tmp = self.min;
                self.min += 1;
                return tmp;
            }
        }


        let num = self.queue.pop();
        let ret = num.unwrap().0;
        self.contains.remove(&ret);

        ret
    }
    
    fn add_back(&mut self, num: i32) {
        if num >= self.min {
            return;
        }

        // already added
        if self.contains.contains(&num) {
            return;
        }

        self.contains.insert(num);
        self.queue.push(Reverse(num));
    }
}

mod tests {
    use super::SmallestInfiniteSet;

    #[test]
    fn test() {
        let mut ss = SmallestInfiniteSet::new();
        ss.add_back(2);
        assert_eq!(1, ss.pop_smallest());
        assert_eq!(2, ss.pop_smallest());
        assert_eq!(3, ss.pop_smallest());
        ss.add_back(1);
        assert_eq!(1, ss.pop_smallest());
        assert_eq!(4, ss.pop_smallest());
        assert_eq!(5, ss.pop_smallest());
    }
}
