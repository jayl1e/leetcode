use  super::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
enum Stage {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
}

pub struct BSTIterator {
    yield_from: Option<Box<BSTIterator>>,
    node: Option<Rc<RefCell<TreeNode>>>,
    stage: Stage,
    staged: Option<i32>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            yield_from: None,
            node: root.clone(),
            stage: Stage::S1,
            staged: None,
        }
    }

    pub fn next(&mut self) -> i32 {
        match self.staged.take() {
            Some(v) => v,
            None => self.maybe_next().unwrap_or(0),
        }
    }

    pub fn has_next(&mut self) -> bool {
        if self.staged.is_none() {
            self.staged = self.maybe_next();
        }
        self.staged.is_some()
    }

    fn maybe_next(&mut self) -> Option<i32> {
        match self.stage {
            Stage::S1 => {
                if self.node.is_none() {
                    return None;
                } else {
                    self.yield_from = Some(Box::new(BSTIterator::new(
                        self.node.as_ref().unwrap().borrow().left.clone(),
                    )));
                    self.stage = Stage::S2;
                    return self.maybe_next();
                }
            }
            Stage::S2 => match self.yield_from.as_mut().unwrap().maybe_next() {
                None => {
                    self.stage = Stage::S3;
                    return self.maybe_next();
                }
                x => {
                    return x;
                }
            },
            Stage::S3 => {
                self.stage = Stage::S4;
                return Some(self.node.as_ref().unwrap().borrow().val);
            }
            Stage::S4 => {
                self.yield_from = Some(Box::new(BSTIterator::new(
                    self.node.as_ref().unwrap().borrow().right.clone(),
                )));
                self.stage = Stage::S5;
                return self.maybe_next();
            }
            Stage::S5 => match self.yield_from.as_mut().unwrap().maybe_next() {
                None => {
                    self.stage = Stage::S6;
                    return self.maybe_next();
                }
                x => {
                    return x;
                }
            },
            Stage::S6 => {
                return None;
            }
        };
    }
}
