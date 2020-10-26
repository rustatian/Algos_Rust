// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//
// push(x) -- Push element x onto stack.
// pop() -- Removes the element on top of the stack.
// top() -- Get the top element.
// getMin() -- Retrieve the minimum element in the stack.
//
//
//
// Example:
//
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin();   --> Returns -3.
// minStack.pop();
// minStack.top();      --> Returns 0.
// minStack.getMin();   --> Returns -2.

#[derive(Default, Copy, Clone)]
struct Node {
    value: i32,
    min_value: i32,
}

struct MinStack {
    data: Box<[Node]>,
    len: usize,
}


// 4 ms	5.3 MB
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            data: Box::from([Node::default(); 10]),
            len: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let mut n = Node::default();
        if self.len == 0 {
            n.value = x;
            n.min_value = x;
            self.len += 1;
            self.data[self.len] = n;
            return;
        }

        n.value = x;
        n.min_value = x;

        let v = self.get_min();
        if x >= self.get_min() {
            n.min_value = v;
        }

        self.len += 1;
        self.data[self.len] = n;
    }

    fn pop(&mut self) {
        self.len -= 1;
    }

    fn top(&self) -> i32 {
        self.data[self.len].value
    }

    fn get_min(&self) -> i32 {
        self.data[self.len].min_value
    }
}

struct MidStackLC {
    data: Vec<i32>,
    min_stack: Vec<[i32; 2]>,
}

// 4 ms	5.4 MB
impl MidStackLC {
    /** initialize your data structure here. */
    fn new() -> Self {
        MidStackLC {
            data: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);

        if self.min_stack.is_empty() || x < self.min_stack.last().unwrap()[0] {
            self.min_stack.push([x, 1]);
        } else if x == self.min_stack.last().unwrap()[0] {
            self.min_stack.last_mut().unwrap()[1] += 1;
        }
    }

    fn pop(&mut self) {
        if self.data.last().unwrap() == &self.min_stack.last().unwrap()[0] {
            self.min_stack.last_mut().unwrap()[1] -= 1;
        }

        if self.min_stack.last().unwrap()[1] == 0 {
            self.min_stack.pop();
        }
        self.data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().unwrap()[0]
    }
}

#[test]
fn tests() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);


    let mut min_stack = MinStack::new();
    min_stack.push(2147483646);
    min_stack.push(2147483646);
    min_stack.push(2147483647);
    assert_eq!(min_stack.top(), 2147483647);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 2147483646);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 2147483646);
    min_stack.pop();
    min_stack.push(2147483647);
    assert_eq!(min_stack.top(), 2147483647);
    assert_eq!(min_stack.get_min(), 2147483647);
    min_stack.push(-2147483648);
    assert_eq!(min_stack.top(), -2147483648);
    assert_eq!(min_stack.get_min(), -2147483648);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 2147483647);


    let mut min_stack = MidStackLC::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);

    let mut min_stack = MidStackLC::new();
    min_stack.push(2147483646);
    min_stack.push(2147483646);
    min_stack.push(2147483647);
    assert_eq!(min_stack.top(), 2147483647);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 2147483646);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 2147483646);
    min_stack.pop();
    min_stack.push(2147483647);
    assert_eq!(min_stack.top(), 2147483647);
    assert_eq!(min_stack.get_min(), 2147483647);
    min_stack.push(-2147483648);
    assert_eq!(min_stack.top(), -2147483648);
    assert_eq!(min_stack.get_min(), -2147483648);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 2147483647);
}