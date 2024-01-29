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
  fn from(xs: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for x in xs.iter().rev(){
        head = Some(Box::new(ListNode{val:*x, next : head}))
    };
    head
  }
}



struct Solution;
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match &head{
            None=>{head},
            Some(node) => {
                match &node.next {
                    None=>{head},
                    Some(nxt)=>{
                        let (left, right) = Self::split(head);
                        let left = Self::sort_list(left);
                        let right = Self::sort_list(right);
                        Self::merge(left,right)
                    }
                }
            }
        }
    }


    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>){
        let mid = Self::find_mid(&head);
        let right = unsafe {
            (*(mid as (*const Option<Box<ListNode>>) as (*mut Option<Box<ListNode>>))).take()
        };
        (head,right)
    }


    fn find_mid(mut head: & Option<Box<ListNode>>) -> & Option<Box<ListNode>>{
        let mut slow = head;
        let mut fast =  head;
        loop{
            if fast.is_none() || fast.as_ref().unwrap().next.is_none(){
                return slow;
            }
            slow = & slow.as_ref().unwrap().next;
            fast = & fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
    }

    fn merge(mut left: Option<Box<ListNode>>, mut right: Option<Box<ListNode>>)->Option<Box<ListNode>>{
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        while left.is_some() && right.is_some(){
            if left.as_ref().unwrap().val <= right.as_ref().unwrap().val{
                let t = left.as_mut().unwrap().next.take();
                tail.next = left;
                left = t;
            }else{
                let t: Option<Box<ListNode>> = right.as_mut().unwrap().next.take();
                tail.next = right;
                right = t;
            }
            tail = tail.next.as_mut().unwrap()
        }
        if left.is_some(){
            tail.next = left
        }else {
            tail.next = right
        }
        head.next
    }


}

fn main(){
    let s = Solution::sort_list(ListNode::from(&vec![1,3,5,2,4]));
    println!("{:?}",s);
}