struct Solution;
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&num| {
                let mut resident = num;
                loop {
                    let digit = resident % 10;
                    if digit == 0 || num % digit != 0 {
                        return false;
                    }
                    resident /= 10;
                    if resident == 0 {
                        break;
                    }
                }
                true
            })
            .collect()
    }
}
fn main() {
    let rt = Solution::self_dividing_numbers(47, 85);
    assert_eq!(vec![48, 55, 66, 77], rt)
}
