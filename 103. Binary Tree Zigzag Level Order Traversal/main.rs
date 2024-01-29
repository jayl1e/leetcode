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
use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q:VecDeque<Rc<RefCell<TreeNode>>> =VecDeque::new();
        let mut rt = vec![];
        if root.is_none(){
            return rt;
        }
        q.push_back(root.unwrap());
        let mut reverse = false;
        while !q.is_empty() {
            let rowc = q.len();
            let mut row = vec![];
            for _ in 0..rowc{
                let node = q.pop_front().unwrap();
                if let Some(l) = node.borrow().left.clone(){
                    q.push_back(l);
                }
                if let Some(r) = node.borrow().right.clone(){
                    q.push_back(r);
                }
                row.push(node.borrow().val);
            }
            if reverse{
                row.reverse();
            }
            rt.push(row);
            reverse = !reverse;
        }
        rt
    }
}

fn main(){

}