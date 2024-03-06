struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(mut s: String) -> String {
        let buf = unsafe {
            s.as_bytes_mut()
        };
        buf.sort();
        let mut idx = 0;
        while idx<buf.len() {
            if buf[idx] == b'1'{
                break;
            }
            idx+=1;
        }
        buf[idx] = buf[0];
        buf[0] = b'1';
        buf.reverse();
        s
    }
}

fn main(){
    let s = Solution::maximum_odd_binary_number("1011".to_string());
    println!("{s}");
}