
struct Solution;

use core::cmp::max;
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counter:[i32;26] = [0;26];
        for c in tasks{
            counter[(c as u8-b'A') as usize] += 1;
        };
        let sum:i32 = counter.iter().sum();
        let most:i32 = *counter.iter().max().unwrap();
        let max_fill = (most-1)*(n+1) + counter.into_iter().filter(|x|{*x==most}).count() as i32;
        max(sum, max_fill)
    }
}

fn main(){
    let tasks = ['A','A','A', 'B','B','B'].to_vec();
    let ivls = Solution::least_interval(tasks, 3);
    assert_eq!(10,ivls);
}