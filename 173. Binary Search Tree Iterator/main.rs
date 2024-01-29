use std::cell::RefCell;
use std::rc::Rc;
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
            right: None,
        }
    }
    
}

mod stack;
mod chain_generator;
mod vec;

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

pub fn new_opt(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}
fn main() {
    let node7 = new_opt(7);
    let node3 = new_opt(3);
    let node15 = new_opt(15);
    node7.as_ref().unwrap().borrow_mut().left = node3;
    node7.as_ref().unwrap().borrow_mut().right = node15;
    let mut iter = vec::BSTIterator::new(node7);
    while iter.has_next() {
        println!("{}", iter.next());
    }
}
