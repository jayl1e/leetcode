struct Solution;
use core::cmp::min;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![u32::MAX;(amount+1) as usize];
        dp[0] = 0;
        for c in coins{
            for idx in 1..= amount{
                let older = idx-c;
                if older >=0 {
                    dp[idx as usize] = min(dp[idx as usize]-1, dp[older as usize])+1;
                }
            }
        }
        if dp[amount as usize] == u32::MAX{
            -1
        }else{
            dp[amount as usize] as i32
        }
    }
}

fn main(){
    let coins = vec![2];
    let amount = 3;
    let n = Solution::coin_change(coins, amount);
    println!("coin size is {}", n);
}