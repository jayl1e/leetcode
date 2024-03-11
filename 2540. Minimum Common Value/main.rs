

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut n1 = nums1.into_iter();
        let mut n2 = nums2.into_iter();
        let mut x1 = n1.next();
        let mut x2 = n2.next();
        while let (Some(ref v1), Some(ref v2)) = (x1,x2) {
            match v1.cmp(v2) {
                Ordering::Equal=>{return *v1;},
                Ordering::Less=>{
                    x1 = n1.next();
                },
                Ordering::Greater=>{
                    x2 = n2.next();
                }
            }
        }
        -1
    }
}

fn main(){
    let v1 =vec! [1,2,3];
    let v2 = vec![2,4];
    let rt = Solution::get_common(v1, v2);
    assert_eq!(2,rt);
}