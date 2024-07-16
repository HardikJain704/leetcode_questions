use std::collections::VecDeque;

struct MyStack {
  q: VecDeque<i32>,

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

   impl MyStack {
    fn new() -> Self {
         MyStack { q: VecDeque::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.q.push_back(x);
        for _ in 0..self.q.len() - 1 {
            let val = self.q.pop_front().unwrap();
            self.q.push_back(val);
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
        
    }
    
    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.q.is_empty()
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_top() {
        let mut stack = MyStack::new();
        stack.push(1);
        assert_eq!(stack.top(), 1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
    }

    #[test]
    fn test_pop() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    fn test_empty() {
        let mut stack = MyStack::new();
        assert!(stack.empty());
        stack.push(1);
        assert!(!stack.empty());
        stack.pop();
        assert!(stack.empty());
    }

    #[test]
    fn test_top() {
        let mut stack = MyStack::new();
        stack.push(1);
        assert_eq!(stack.top(), 1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        stack.pop();
        assert_eq!(stack.top(), 1);
    }
}

fn main() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(5);

    println!("{:?}" , stack.top());
    println!("{:?}" , stack.pop());
    println!("{:?}" , stack.empty());

}
