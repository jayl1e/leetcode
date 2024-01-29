use core::cmp::min;
struct MinStack {
    s:Vec<i32>,
    m:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self{
            s:Vec::new(),
            m:Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.s.push(val);
        match self.m.last() {
            None=>{self.m.push(val)},
            Some(v)=>{
                self.m.push(min(*v,val))
            }
        }
    }
    
    fn pop(&mut self) {
        self.s.pop();
        self.m.pop();
    }
    
    fn top(&self) -> i32 {
        *self.s.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.m.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main(){

}