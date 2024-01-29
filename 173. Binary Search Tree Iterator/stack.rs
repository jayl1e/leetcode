use  super::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
pub struct BSTIterator {
    stack: Vec<(Rc<RefCell<TreeNode>>, bool)>,
}

impl BSTIterator {
    pub fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut t = Self { stack: Vec::new() };
        Self::add_left_all(&mut t, root);
        t
    }

    fn add_left_all(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while root.is_some() {
            self.stack.push((root.as_ref().unwrap().clone(), false));
            root = root.unwrap().borrow().left.clone();
        }
    }

    fn pop_until_unvisited(&mut self) {
        while let Some((_, visited)) = self.stack.last() {
            if *visited {
                self.stack.pop();
            } else {
                break;
            }
        }
    }

    pub fn next(&mut self) -> i32 {
        self.has_next();
        let v = self.stack.last_mut().unwrap();
        v.1 = true;
        v.0.borrow().val
    }

    pub fn has_next(&mut self) -> bool {
        loop {
            if self.stack.last().is_none() {
                break;
            }
            let (node, visited) = self.stack.last().unwrap();
            if !visited {
                return true;
            }
            let node = node.clone();
            if node.borrow().right.is_some() {
                self.add_left_all(node.borrow().right.clone());
            } else {
                self.pop_until_unvisited();
            }
        }
        !self.stack.is_empty()
    }
}