struct Solution;
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut rt = left&right;
        let mut rg  = right - left;
        let mut flag = 1;
        while rg!=0 {
            rt =  rt & !flag;
            rg >>= 1;
            flag <<= 1;
        }
        rt
    }
}
fn main(){}