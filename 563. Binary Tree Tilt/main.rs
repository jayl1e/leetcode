struct Solution;

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
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::find_tilt_rc(&root).0 as i32
    }
    fn find_tilt_rc(root: &Option<Rc<RefCell<TreeNode>>>) -> (u32,i32){
        match root {
            None=>(0,0),
            Some(node)=>{
                let (l_t, l_s) = Self::find_tilt_rc(&node.borrow().left);
                let (r_t, r_s) = Self::find_tilt_rc(&node.borrow().right);
                let tilt = l_s.abs_diff(r_s);
                (l_t+r_t+tilt,l_s+r_s+node.borrow().val)
            }
        }
    }
}

fn main(){
    let mut node1 = TreeNode::new(1);
    let node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    node1.left = Some(Rc::new(RefCell::new(node2)));
    node1.right = Some(Rc::new(RefCell::new(node3)));
    let node1 = Some(Rc::new(RefCell::new(node1)));
    let tilt = Solution::find_tilt(node1);
    assert_eq!(1, tilt);
}