
struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut rt: Vec<i32> = nums.into_iter().map(|x|{x*x}).collect();
        rt.sort();
        rt
    }
}

fn main(){
    let nums = [-7,-3,2,3,11].to_vec();
    let rt = Solution::sorted_squares(nums);
    assert_eq!([4, 9,9,49,121], *rt);
}