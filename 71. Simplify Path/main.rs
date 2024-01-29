use std::ops::AddAssign;

struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for word in path.split('/'){
            match word {
                "" | "."=>{},
                ".." =>{stack.pop();},
                w =>{stack.push(w);}
            }
        }
        let mut rt ="/".to_owned();
        rt.push_str(&stack.join("/"));
        rt
    }
}

fn main(){
    let rt = Solution::simplify_path("/home//foo/".to_owned());
    println!("{}", rt);
}