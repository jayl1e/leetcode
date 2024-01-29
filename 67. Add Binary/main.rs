struct Solution;
use std::cmp::max;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let longest = max(a.len(), b.len());
        let mut sum = Vec::with_capacity(longest + 1);
        let mut carry = 0;
        let mut left = a.into_bytes().into_iter().rev();
        let mut right = b.into_bytes().into_iter().rev();
        for _ in 0..longest {
            if let Some(b'1') = left.next() {
                carry += 1;
            }
            if let Some(b'1') = right.next() {
                carry += 1;
            }
            if carry & 0x1 == 1 {
                sum.push(b'1')
            } else {
                sum.push(b'0')
            }
            carry >>= 1;
        }
        if carry == 1 {
            sum.push(b'1');
        }
        sum.reverse();
        String::from_utf8(sum).unwrap()
    }
}

fn main() {
    let left = String::from("1010");
    let right = String::from("1011");
    let s = Solution::add_binary(left, right);
    println!("{}", s);
}
