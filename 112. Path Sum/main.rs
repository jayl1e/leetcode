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
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node)=>{
                let r = target_sum-node.borrow().val;
                Self::path_sum_rec(node, r)
            },
            None=>{
                false
            }
        }
        
    }
    fn path_sum_rec(node: Rc<RefCell<TreeNode>>, resident: i32)->bool{
        if node.borrow().left.is_none() && node.borrow().right.is_none(){
            return resident == 0;
        }
        if let Some(left) = &node.borrow().left {
            if Self::path_sum_rec(left.clone(), resident-left.borrow().val){
                return true;
            }
        }
        if let Some(right) = &node.borrow().right {
            if Self::path_sum_rec(right.clone(), resident-right.borrow().val){
                return true;
            }
        }
        false
    }
}

fn main(){

}