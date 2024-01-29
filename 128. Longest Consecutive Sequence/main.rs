struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set:HashSet<i32> = nums.into_iter().collect();
        set.iter().filter(|x|{!set.contains(&(*x-1))}).
            map(|x|{(*x..).into_iter().take_while(|v|{set.contains(v)}).count()}).
            max().map_or(0, |x|{x as i32})
    }
}

fn main(){
    let rt =Solution::longest_consecutive(vec![100,4,200,1,3,2]);
    assert_eq!(4,rt);
}