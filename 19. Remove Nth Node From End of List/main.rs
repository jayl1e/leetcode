struct Solution;

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

fn list_from_vec(xs: Vec<i32>)->Option<Box<ListNode>>{
  let mut head = None;
  for i in xs.into_iter().rev(){
    head = Some(Box::new(ListNode{val:i, next: head}))
  }
  head
}

fn list_to_vec(mut head: Option<Box<ListNode>>)->Vec<i32>{
  let mut rt = vec![];
  while let Some(val) = head {
      rt.push(val.val);
      head = val.next;
  }
  rt
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
      let phead = Some(Box::new(ListNode{next: head, val:0}));
      let mut slow=&phead;
      let mut fast = slow;
      for _ in 0..=n{
        if let Some(val) = fast{
          fast = &val.next;
        }
      }
      while let Some(fst) = fast {
        fast = &fst.next;
        slow = &slow.as_ref().unwrap().next;
      }
      let p = slow as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
      let sp = unsafe {
        p.as_mut().unwrap()
      };
      let next = sp.as_mut().unwrap().next.as_mut().unwrap().next.take();
      sp.as_mut().unwrap().next= next;
      phead.unwrap().next.take()
    }
}

fn main(){
    let list = vec![1,2];
    let n = 1;
    let head = list_from_vec(list);
    let head = Solution::remove_nth_from_end(head, n);
    let list = list_to_vec(head);
    assert_eq!(vec![1], list);
}