struct Solution;
impl Solution{
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map_or(0, |x|{x.len() as i32})
    }
}

fn main(){
    let s = "   fly me   to   the moon  ".to_string();
    let rt = Solution::length_of_last_word(s);
    println!("{rt}");
}