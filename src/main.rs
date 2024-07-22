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
      val,
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut prev = &mut dummy;

        while let Some(mut current) = prev.next.take() {
            let mut is_duplicate = false;

            while let Some(next) = current.next.as_ref() {
                if next.val == current.val {
                    is_duplicate = true;
                    current.next = current.next.take().unwrap().next;
                } else {
                    break;
                }
            }

            if is_duplicate {
                prev.next = current.next;
            } else {
                prev.next = Some(current);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}


fn vec_to_linked_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in v.iter() {
        current = &mut current.insert(Box::new(ListNode::new(val))).next;
    }

    head
}

fn linked_list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(n) = node {
        v.push(n.val);
        node = n.next;
    }
    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let head = vec_to_linked_list(vec![1, 2, 3, 3, 4, 4, 5]);
        let res = Solution::delete_duplicates(head);
        let result_vec = linked_list_to_vec(res);
        assert_eq!(result_vec, vec![1,2,5]);

        let head = vec_to_linked_list(vec![1, 1, 1, 2, 3]);
        let res = Solution::delete_duplicates(head);
        let result_vec = linked_list_to_vec(res);
        assert_eq!(result_vec, vec![2, 3]);

        let head = vec_to_linked_list(vec![1, 1, 1]);
        let res = Solution::delete_duplicates(head);
        let result_vec = linked_list_to_vec(res);
        assert_eq!(result_vec, vec![]);

        let head = vec_to_linked_list(vec![1, 2, 3, 4, 5]);
        let res = Solution::delete_duplicates(head);
        let result_vec = linked_list_to_vec(res);
        assert_eq!(result_vec, vec![1, 2, 3, 4, 5]);
    }
}


fn main() {
    let head = vec_to_linked_list(vec![1,2,3,3,4,4,5,]);

    let res =  Solution::delete_duplicates(head);

    let res_vec = linked_list_to_vec(res);

    println!("{:?}" , res_vec);


}
