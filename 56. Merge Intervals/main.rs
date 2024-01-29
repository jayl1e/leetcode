struct Solution;
use std::cmp::max;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut rt_idx = 0;
        for current in 1..intervals.len(){
            if intervals[current][0]<=intervals[rt_idx][1]{
                intervals[rt_idx][1] = max(intervals[current][1],intervals[rt_idx][1])
            }else{
                rt_idx+=1;
                intervals[rt_idx][0] = intervals[current][0];
                intervals[rt_idx][1] = intervals[current][1];
            }
        };
        intervals.truncate(rt_idx+1);
        intervals
    }
}

fn tuples_to_vecs(intervals: Vec<(i32,i32)>)-> Vec<Vec<i32>>{
    intervals.into_iter().map(|(left,right)|{vec![left,right]}).collect()
}

fn main(){
    let vs = tuples_to_vecs(vec![(1,3),(2,6),(8,10),(15,18)]);
    let rt = Solution::merge(vs);
    println!("{:?}",rt);

}