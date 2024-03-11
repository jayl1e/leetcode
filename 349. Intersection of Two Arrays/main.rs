struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1:HashSet<i32> = nums1.into_iter().collect();
        let s2:HashSet<i32> = nums2.into_iter().collect();
        s1.intersection(&s2).into_iter().map(|x|{*x}).collect()
    }
}

fn main(){
    let rt = Solution::intersection(vec![1,2,2,1], vec![2,2]);
    assert_eq!(vec![2], rt);
}