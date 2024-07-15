// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy  = Box::new(ListNode {
            val: -1, 
            next: head
        });

        let mut curr = &mut dummy;

        while let Some(mut node) = curr.next.take(){
            if node.val == val {
                curr.next = node.next.take();

            } else {
                curr.next = Some(node);
                curr = curr.next.as_mut().unwrap();


            }
            
        }
        dummy.next
}


// Helper function to convert a vector to a linked list
fn vec_to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
   
     for &val in vec.iter().rev() {
        let new_node = Box::new(ListNode { val, next: head });
         head = Some(new_node);

    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        // Test case 1: normal list with nodes to remove
        let head = vec_to_linked_list(vec![1, 2, 6, 3, 4, 5, 6]);
        let val = 6;
        let result = remove_elements(head, val);

        let expected = vec_to_linked_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_elements_empty() {
        // Test case 2: empty list
        let head = None;
        let val = 5;
        let result = remove_elements(head, val);
        assert_eq!(result, None);
    }

    #[test]
    fn test_remove_elements_no_match() {
        // Test case 3: list with no nodes to remove
        let head = vec_to_linked_list(vec![1, 2, 3, 4]);
        let val = 5;
        let result = remove_elements(head, val);

        let expected = vec_to_linked_list(vec![1, 2, 3, 4]);
        assert_eq!(result, expected);
    }
}


fn main() {
   let head = vec![1,2,6,3,4,5,6];
   let val = 6;

   let linked_list = vec_to_linked_list(head);
   let res = remove_elements(linked_list, val);

   println!("{:?}" , res);

}
