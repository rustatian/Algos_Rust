struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct MyLinkedList {
    head: Option<Box<Node>>
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { head: None }
    }
    
    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut curr_idx = 0;
        while self.head.is_some() {
            if curr_idx == index {
                if let Some(val) = &self.head {
                    return val.val;
                }
            }
        }
       -1
    }
    
    fn add_at_head(&self, val: i32) {
        
    }
    
    fn add_at_tail(&self, val: i32) {
        
    }
    
    fn add_at_index(&self, index: i32, val: i32) {
        
    }
    
    fn delete_at_index(&self, index: i32) {
        
    }
}
