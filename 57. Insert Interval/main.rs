struct Solution;

use std::cmp::{min,max};
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut to_insert = Some(new_interval);
        let mut rt = Vec::with_capacity(intervals.len());
        for target in intervals{
            match to_insert.take() {
                None=>{rt.push(target)},
                Some( ni)=>{
                    if ni[0]<=target[1]{
                        if ni[1]>=target[0]{
                            // overlap
                            to_insert = Some(vec![min(ni[0], target[0]), max(ni[1],target[1])]);
                        }else{
                            // ni is left
                            rt.push(ni);
                            rt.push(target);
                        }
                    }else{
                        // ni is right
                        rt.push(target);
                        to_insert = Some(ni);
                    }
                }
            }
        }
        if let Some(ni) = to_insert{
            rt.push(ni);
        }
        rt
    }
}

fn main(){
    let intervals = vec![vec![1,3],vec![6,9]];
    let new_interval = vec![2,5];
    let rt = Solution::insert(intervals, new_interval);
    assert_eq!(vec![vec![1,5],vec![6,9]],rt);
}