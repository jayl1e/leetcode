use  super::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;


pub struct BSTIterator {
    vs: Vec<i32>,
    pos: usize,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut v = vec![];
        Self::dfs(root, &mut v);
        Self{
            vs: v,
            pos:0
        }
    }

    pub fn next(&mut self) -> i32 {
        if self.has_next(){
            let v = self.vs[self.pos];
            self.pos+=1;
            v
        }else{
            0
        }
       
    }

    pub fn has_next(&self) -> bool {
        return self.pos<self.vs.len();
    }
    
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>){
        match node{
            None=>{},
            Some(node)=>{
                Self::dfs(node.borrow().left.clone(), v);
                v.push(node.borrow().val);
                Self::dfs(node.borrow().right.clone(), v);
            }
        }
    }
}