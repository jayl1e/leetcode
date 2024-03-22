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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut v = vec![];
        while let Some(node)=head{
            v.push(node.val);
            head = node.next;
        }
        for i in 0 .. (v.len()/2){
            if v[i] != v[v.len()-1-i]{
                return false;
            }
        }
        return true;
    }
}

fn vec2list(vec: Vec<i32>)->Option<Box<ListNode>>{
  let mut head = None;
  for v in vec.into_iter().rev(){
    head = Some(Box::new(ListNode{val:v, next : head}))
  }
  head
}

fn main(){
  let h = vec2list(vec![2,1,3]);
  let r = Solution::is_palindrome(h);
  assert_eq!(false,r);
}