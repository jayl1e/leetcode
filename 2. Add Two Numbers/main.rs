use std::ops::{Deref, DerefMut};

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

fn list_from_str(v: &str)->Option<Box<ListNode>>{
    let mut phead = ListNode::new(0);
    let mut tail = &mut phead;
    for c in v.bytes(){
        let newtail = Some(Box::new(ListNode::new((c-b'0').into())));
        tail.next = newtail;
        tail = tail.next.as_mut().unwrap();
    }
    phead.next
}

fn list_to_str(mut l: Option<Box<ListNode>>)->String{
    let mut rt = String::new();
    while let Some(node) = l {
        rt.push((b'0' + node.val as u8).into());
        l = node.next;
    }
    rt
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut phead = ListNode::new(0);
        let mut tail = &mut phead;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let left = match l1 {
                None=>0,
                Some(node)=>{
                    l1 = node.next;
                    node.val
                }
            };
            let right = match l2 {
                None=>0,
                Some(node)=>{
                    l2 = node.next;
                    node.val
                }
            };
            let mut cur = left+right+ carry;
            if cur>9{
                cur -= 10;
                carry = 1;
            }else{
                carry = 0;
            }
            let newtail = Some(Box::new(ListNode::new(cur)));
            tail.next = newtail;
            tail = tail.next.as_mut().unwrap();
        };
        if carry!=0{
            let newtail = Some(Box::new(ListNode::new(carry)));
            tail.next = newtail;
            tail = tail.next.as_mut().unwrap();
        }
        phead.next
    }
}

fn main(){
    let l = "243".to_string();
    let r = "568".to_string();
    let add = Solution::add_two_numbers(list_from_str(&l), list_from_str(&r));
    let add = list_to_str(add);
    assert_eq!("7021", &add);
}