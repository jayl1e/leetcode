use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

struct ListNode{
    val:i32,
    prev:Option<Rc<RefCell<ListNode>>>,
    next:Option<Rc<RefCell<ListNode>>>,
}

struct LRUCache {
    kp: HashMap<i32,Rc<RefCell<ListNode>>>,
    head:Rc<RefCell<ListNode>>,
    tail:Rc<RefCell<ListNode>>,
    capacity:usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        let t=LRUCache{
            kp:HashMap::new(),
            head:Rc::new(RefCell::new(ListNode{val:0, prev:None, next:None})),
            tail:Rc::new(RefCell::new(ListNode{val:0, prev:None, next:None})),
            capacity:capacity as usize,
        };
        t.head.as_ref().borrow_mut().next = Some(Rc::clone(&t.tail));
        t.tail.as_ref().borrow_mut().prev = Some(Rc::clone(&t.head));
        t
    }
    
    pub fn get(&self, key: i32) -> i32 {
        match self.kp.get(&key)  {
            None=>0,
            Some(p)=>{
                {
                    let next = p.borrow().next.clone();
                    p.borrow_mut().prev = next.clone();
                    next.unwrap().borrow_mut().prev = p.borrow().prev.clone();
                    self.head.borrow().next.as_ref().unwrap().borrow_mut().prev.replace(p.clone());
                    p.borrow_mut().next = self.head.borrow().next.clone();
                    self.head.borrow_mut().next = Some(p.clone());
                    p.borrow_mut().prev.replace(self.head.clone());
                }
                p.borrow().val
            }
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if self.kp.len()<self.capacity{
            let p = Rc::new(RefCell::new(ListNode{val:value, prev:None, next:None}));
            self.head.borrow().next.as_ref().unwrap().borrow_mut().prev.replace(p.clone());
            p.borrow_mut().next = self.head.borrow().next.clone();
            self.head.borrow_mut().next = Some(p.clone());
            p.borrow_mut().prev.replace(self.head.clone());
            self.kp.insert(key, p);
        }
    }
}

impl Display for LRUCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = vec![];
        let mut p = self.head.borrow().next.clone();
        while let Some(t) = p{
            values.push(t.borrow().val);
            p = t.borrow().next.clone();
        }
        write!(f, "{:?}", values)
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main(){
    let mut t = LRUCache::new(2);
    println!("{}", t.capacity);
    println!("{}", t.get(3));
    t.put(3,32);
    t.put(2,22);
    println!("{}", t.get(3));
    println!("{}", t);
    println!("done");
}