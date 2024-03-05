struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut left = 0;
        let mut right = bs.len();
        while left + 1 < right {
            if bs[left] != bs[right - 1] {
                break;
            }
            let c = bs[left];
            while left < right && bs[left] == c {
                left += 1;
            }
            while left < right && bs[right - 1] == c {
                right -= 1;
            }
        }
        (right - left) as i32
    }
}

fn main() {
    let s = "aabccabba".to_string();
    let c = Solution::minimum_length(s);
    assert_eq!(3, c);

    let s = "cabaabac".to_string();
    let c = Solution::minimum_length(s);
    assert_eq!(0, c);
}
