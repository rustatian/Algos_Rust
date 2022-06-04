/*
Design a max stack data structure that supports the stack operations and supports finding the stack's maximum element.

Implement the MaxStack class:

MaxStack() Initializes the stack object.
void push(int x) Pushes element x onto the stack.
int pop() Removes the element on top of the stack and returns it.
int top() Gets the element on the top of the stack without removing it.
int peekMax() Retrieves the maximum element in the stack without removing it.
int popMax() Retrieves the maximum element in the stack and removes it. If there is more than one maximum element, only remove the top-most one.


Example 1:

Input
["MaxStack", "push", "push", "push", "top", "popMax", "top", "peekMax", "pop", "top"]
[[], [5], [1], [5], [], [], [], [], [], []]
Output
[null, null, null, null, 5, 5, 1, 5, 1, 5]

Explanation
MaxStack stk = new MaxStack();
stk.push(5);   // [5] the top of the stack and the maximum number is 5.
stk.push(1);   // [5, 1] the top of the stack is 1, but the maximum is 5.
stk.push(5);   // [5, 1, 5] the top of the stack is 5, which is also the maximum, because it is the top most one.
stk.top();     // return 5, [5, 1, 5] the stack did not change.
stk.popMax();  // return 5, [5, 1] the stack is changed now, and the top is different from the max.
stk.top();     // return 1, [5, 1] the stack did not change.
stk.peekMax(); // return 5, [5, 1] the stack did not change.
stk.pop();     // return 1, [5] the top of the stack and the max element is now 5.
stk.top();     // return 5, [5] the stack did not change.
 */

#[derive(Debug)]
struct MaxStack<'a> {
    cap: usize,
    len: usize,
    max: i32,
    vals: &'a mut [i32],
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */

impl MaxStack<'_> {
    fn new() -> Self {
        Self {
            cap: 0,
            len: 0,
            max: -1,
            vals: &mut [],
        }
    }

    fn push(&mut self, x: i32) {
        // match self.vals {
        //     None => {
        //         self.vals = Option::from(Box::new([x]));
        //     }
        //     Some(val) => {
        //
        //     }
        // }

        self.vals.copy_from_slice(&[x]);
        // self.vals[self.len+1] = x;
        // self.vals[self.len + 1] = x;
        if x > self.max {
            self.max = x;
        }
    }

    fn pop(&self) -> i32 {
        1
    }

    fn top(&self) -> i32 {
        1
    }

    fn peek_max(&self) -> i32 {
        1
    }

    fn pop_max(&self) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::max_stack::MaxStack;

    #[test]
    fn test1() {
        let mut obj = MaxStack::new();
        obj.push(10);
        obj.push(10);
        obj.push(10);
        obj.push(10);
        obj.push(10);
        // let ret_2: i32 = obj.pop();
        // let ret_3: i32 = obj.top();
        // let ret_4: i32 = obj.peek_max();
        // let ret_5: i32 = obj.pop_max();
    }
}
