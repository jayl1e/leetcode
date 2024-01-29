
use std::rc::Rc;
use std::cell::RefCell;

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
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_recursive(&inorder, &postorder)
    }
    fn build_tree_recursive(inorder: &[i32], postorder: &[i32]) ->  Option<Rc<RefCell<TreeNode>>>{
        if inorder.len() == 0{
            return  None;
        }
        let mid = postorder[postorder.len()-1];
        let pos = inorder.iter().position(|&x|{x==mid}).unwrap();
        let in_l = &inorder[..pos];
        let in_r = &inorder[pos+1..];
        let po_l = &postorder[..in_l.len()];
        let po_r = &postorder[in_l.len()..in_l.len()+in_r.len()];
        let left = Self::build_tree_recursive(in_l, po_l);
        let right = Self::build_tree_recursive(in_r, po_r);
        Some(Rc::new(RefCell::new(TreeNode{val:mid, left, right})))
    }
}
fn main() {
    println!("Hello, world!");
}
