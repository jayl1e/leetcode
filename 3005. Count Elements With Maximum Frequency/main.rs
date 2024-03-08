
struct Solution;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut max_freq = 0;
        let mut total_freq = 0;
        let mut counters : [i32;101] = [0;101];
        for n in nums{
            let n = n as usize;
            counters[n]+=1;
            if counters[n]>max_freq{
                max_freq = counters[n];
                total_freq = max_freq;
            }else if counters[n]==max_freq{
                total_freq += max_freq;
            }
        }
        total_freq
    }
}

fn main(){
    let nums = vec![1,2,2,3,1,4];
    let tf = Solution::max_frequency_elements(nums);
    assert_eq!(4,tf);
}