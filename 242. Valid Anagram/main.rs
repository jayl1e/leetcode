struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hm: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|c|{match hm.get_mut(&c) {
            Some(v)=>{*v += 1;},
            None=>{
                hm.insert(c, 1);
            }
        }});
        for c in t.chars(){
            match hm.get_mut(&c) {
                None=>{return false;},
                Some(v)=>{
                    if *v <=0{
                        return false;
                    }else{
                        *v-=1;
                    }
                }
            }
        };
        hm.into_iter().all(|(_c,n)|{n==0})
    }
}

fn main(){
    let rt = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
    println!("result is {rt}");
}