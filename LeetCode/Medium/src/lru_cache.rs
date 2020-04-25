use std::collections::HashMap;

struct AccessBit {
    value: i32,
    accessed: i32,
}

struct LRUCache {
    // key, value
    hash: HashMap<i32, AccessBit>,
    // accessed_bit, key
    access_hash: HashMap<i32, i32>,
    capacity: i32,
    stored: i32,
    curr_index: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            hash: HashMap::with_capacity(capacity as usize),
            access_hash: HashMap::with_capacity(capacity as usize),
            capacity,
            stored: 0,
            curr_index: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(val) = self.hash.get_mut(&key) {
            self.access_hash.remove(&(val.accessed));

            val.accessed = self.curr_index + 1;

            // insert current access_index key and key
            self.access_hash.insert(self.curr_index + 1, key);


            self.curr_index += 1;
            val.value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.stored < self.capacity {
            // if key is in the hash at the moment
            if let Some(val) = self.hash.get_mut(&key) {
                self.access_hash.remove(&val.accessed);
                val.accessed += self.curr_index + 1;

                self.access_hash.insert(val.accessed + 1, key);
                val.value = value;
                self.curr_index += 1;
            } else {
                let ab = AccessBit {
                    accessed: self.curr_index + 1,
                    value,
                };

                let tmp = self.curr_index + 1;
                self.access_hash.insert(tmp, key);

                self.hash.insert(key, ab);
                self.curr_index += 1;
            }
        } else {
            // if key is in the hash at the moment
            if let Some(val) = self.hash.get_mut(&key) {
                self.access_hash.remove(&val.accessed);
                val.accessed += self.curr_index + 1;

                self.access_hash.insert(val.accessed + 1, key);
                val.value = value;
                self.curr_index += 1;
            } else {
                for i in 0..=self.curr_index {
                    // get key by access bit
                    if let Some(val) = self.access_hash.get_mut(&i) {
                        // i here is access bit
                        // val is key for hash

                        let ab = AccessBit {
                            accessed: self.curr_index + 1,
                            value,
                        };

                        // remove LRU value with key
                        self.hash.remove(val);

                        // replace with new
                        self.hash.insert(key, ab);

                        self.curr_index += 1;
                        self.access_hash.remove(&i);

                        break;
                    }
                }
            }
        }

        self.stored += 1;
    }
}

#[test]
fn tests() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}

#[test]
fn tests2() {
    let mut cache = LRUCache::new(2);
    cache.put(2, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(2), 2);
    cache.put(1, 1);
    cache.put(4, 1);
    assert_eq!(cache.get(2), -1);
}

#[test]
fn tests3() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.get(2), -1);
    cache.put(2, 6);
    assert_eq!(cache.get(1), -1);
    cache.put(1, 5);
    cache.put(1, 2);
    assert_eq!(cache.get(1), 2);
    assert_eq!(cache.get(2), 6);
}

#[test]
fn tests4() {
    let mut cache = LRUCache::new(2);
    cache.put(2, 1);
    cache.put(1, 1);
    cache.put(2, 3);
    cache.put(4, 1);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(2), 3);
}

// /**
//  * Your LRUCache object will be instantiated and called as such:
//  * let obj = LRUCache::new(capacity);
//  * let ret_1: i32 = obj.get(key);
//  * obj.put(key, value);
//  */