use super::Solution;

/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// @lc code=start
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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        
        let mut over = 0;
        let mut data = 0;
        let mut list = ListNode::new(0);
        let mut current = &mut list;

        while l1.is_some() || l2.is_some() || over>0 {
            let l1_val = l1.as_ref().map(|node| node.val).unwrap_or(0);
            let l2_val = l2.as_ref().map(|node| node.val).unwrap_or(0);

            data = l1_val+l2_val+over;
            over = data/10;

            let new_list = ListNode::new(data%10);
            current.next = Some(Box::new(new_list));
            current = current.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        list.next
    }
}
// @lc code=end

