struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut mayrt = strs[0].clone();
        let mut first = mayrt.chars();
        let mut prefix_len = 0;
        let mut chars:Vec<_> = strs.iter().map(|x|{x.chars()}).collect();
        'outer:loop{
            let c = first.next();
            match c {
                None=>break,
                Some(c)=>{
                    for is in &mut chars{
                        let ic = is.next();
                        match ic {
                            Some(ric) if ric == c =>{},
                            _=>{break 'outer;}
                        }
                    }
                }
            }
            prefix_len+=1;
        }
        mayrt.truncate(prefix_len);
        mayrt
    }
}

fn main(){
    let strs:Vec<_> = vec!["flowersss","flow","flight"].into_iter().map(|x|{x.to_string()}).collect();
    let prefix = Solution::longest_common_prefix(strs);
    println!("{}", prefix);
}