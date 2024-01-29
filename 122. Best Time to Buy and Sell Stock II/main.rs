struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut rt = 0;
        for i in 0..prices.len()-1{
            let profit = prices[i+1] - prices[i];
            if profit>0{
                rt += profit
            }
        }
        rt
    }
}

fn main(){
    let p = Solution::max_profit(vec![7,1,5,3,6,4]);
    assert_eq!(7,p);
}