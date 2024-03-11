struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, mut s: String) -> String {
        let mut pos = [usize::MAX;128];
        let order = order.as_bytes();
        for i in 0..order.len(){
            pos[order[i] as usize] = i;
        }
        unsafe{
            s.as_bytes_mut().sort_unstable_by_key(|c|{pos[*c as usize]})
        }
        s
    }
}

fn main(){
    let s=Solution::custom_sort_string("bcafg".to_owned(), "abcd".to_owned());
    println!("{s}");
}