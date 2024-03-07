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

struct Solution;

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        while let Some(node) = fast  {
            fast = &node.next;
            if let Some(node) = fast{
                fast = &node.next;
                slow = &slow.as_ref().unwrap().next;
            }
        }
        let slow = slow as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
        let slow = unsafe{
            slow.as_mut().unwrap()
        };
        slow.take()
    }
}

fn main(){

}