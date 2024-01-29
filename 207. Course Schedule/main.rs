use std::{collections::HashMap, ops::Deref};
struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        //let mut prequest = HashMap::new();
        false
    }
}

fn foo(f: &dyn ToString) -> String {
    bar(f)
}

fn bar<T: ToString + ?Sized>(v: &T) -> String {
    v.to_string()
}

fn main() {
    let a = 123;
    let b: Box<dyn ToString> = Box::new(a);
    let v = foo(&*b);
    println!("{}", v)
}
