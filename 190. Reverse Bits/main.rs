
struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut rt:u32 = 0;
        let mut x=x;
        for _ in 0..32{
            rt = (rt<<1) | (x&0x1);
            x>>=1
        }
        rt
    }
}

fn main(){

}