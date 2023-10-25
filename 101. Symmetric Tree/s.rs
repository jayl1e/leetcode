// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

type PTN = Option<Rc<RefCell<TreeNode>>>;
fn isMirror(left:PTN, right:PTN) -> bool{
  if left == right{
      return true
  }
  return false
}
struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      return isMirror(*root.unwrap().left, *root.unwrap().right)
    }
}

fn main(){

}