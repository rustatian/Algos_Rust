use std::collections::{HashMap};
use std::collections::hash_map::Entry;
use std::ops::Add;

struct FirstUnique {
    hash: HashMap<i32, i32>,
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        let mut h = HashMap::new();
        // let mut v = nums;
        for i in nums.iter() {
            h.entry(*i).and_modify(|mut v| *v += 1).or_insert(1);
        }

        let mut v = Vec::new();

        for i in &nums {
            if let Entry::Occupied(mut e) = h.entry(*i) {
                if *e.get() > 1 {
                    //return;
                } else {
                    v.push(*i);
                }
            }
        }


        FirstUnique {
            hash: h,
            data: v,
        }
    }

    fn show_first_unique(&mut self) -> i32 {
        if self.data.is_empty() {
            -1
        } else {
            *self.data.first().unwrap()
        }
    }

    fn add(&mut self, value: i32) {
        match self.hash.entry(value) {
            Entry::Occupied(mut e) => {
                if *e.get() > 1 {} else {
                    if let Some(idx) = self.data.iter().position(|x| *x == value) {
                        self.data.remove(idx);
                    }
                    e.get_mut().add(1);
                }
            }
            Entry::Vacant(e) => {
                e.insert(1);
                self.data.push(value);
            }
        }
    }
}

#[test]
fn tests() {
    let mut fu = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
    assert_eq!(fu.show_first_unique(), -1);
    fu.add(7);
    fu.add(3);
    fu.add(3);
    fu.add(7);
    fu.add(17);
    assert_eq!(fu.show_first_unique(), 17);


    fu = FirstUnique::new(vec![2, 3, 5]);
    assert_eq!(fu.show_first_unique(), 2);
    fu.add(5);
    assert_eq!(fu.show_first_unique(), 2);
    fu.add(2);
    assert_eq!(fu.show_first_unique(), 3);
    fu.add(3);
    assert_eq!(fu.show_first_unique(), -1);

    let mut fu = FirstUnique::new(vec![233]);
    assert_eq!(fu.show_first_unique(), 233);
    fu.add(11);
    assert_eq!(fu.show_first_unique(), 233);
}
// Use doubly Linked list with hashmap of pointers to linked list nodes. add unique number to the linked list.
// When add is called check if the added number is unique then it have to be added to the linked list and if it is repeated remove it from the linked list if exists.
// When showFirstUnique is called retrieve the head of the linked list.

// Use queue and check that first element of the queue is always unique.

// Use set or heap to make running time of each function O(logn).
