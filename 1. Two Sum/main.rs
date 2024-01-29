use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for (idx,i) in nums.into_iter().enumerate(){
            match  map.get(&i) {
                Some(old)=>{
                    return vec![*old, idx as i32];
                },
                None=>{
                    map.insert(target-i, idx as i32);
                }
            }
        };
        vec![]
    }
}

fn main(){
    let v =Solution::two_sum(vec![1,3,5,6], 6);
    println!("{:?}", v);
}
