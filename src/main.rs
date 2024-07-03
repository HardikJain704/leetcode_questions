#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;

        let mut dummy = Box::new(dummy);
        let mut current = dummy.as_mut();

        while let Some(mut slow) = current.next.take(){
            if let Some(mut fast) = slow.next.take(){
                 
               slow.next = fast.next.take();
               fast.next = Some(slow);
               current.next = Some(fast);
               current = current.next.as_mut().unwrap().next.as_mut().unwrap();


            } else {
               current.next = Some(slow);
               break;
            }
            
        }
        dummy.next
}

// Helper function to create a linked list from a vector
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = current;
        current = Some(node);
    }
    current
}

// Helper function to print the linked list
fn print_list(head: Option<Box<ListNode>>) {
    let mut current = head.as_ref();
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next.as_ref();
    }
    println!();
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]

 fn test_swap_pair(){
    let test_cases = vec![
        (vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
        (vec![1, 2, 3, 4, 5], vec![2, 1, 4, 3, 5]),
        (vec![1], vec![1]),
        (vec![], vec![]),
    ];
    for (input, expected) in test_cases {
        let list = vec_to_list(input);
        let res = swap_pairs(list);
        let expected_list = vec_to_list(expected);
        assert_eq!(expected_list, res);
    }
 }
}


fn main() {
    let head =vec! [1,2,3,4];
    let list = vec_to_list(head);
    let res = swap_pairs(list);
    
    print_list(res);

}
