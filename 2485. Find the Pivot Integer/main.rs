struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let p = f64::sqrt((n * n + n) as f64 / 2.0) as i32;
        if (p * p) * 2 == n * n + n {
            p
        } else {
            -1
        }
    }
}

fn main() {
    let p = Solution::pivot_integer(8);
    assert_eq!(6, p)
}
