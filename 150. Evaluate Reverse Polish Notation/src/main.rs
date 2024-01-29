use core::fmt;

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut s = vec![];
    for token in &tokens{
        if let Ok(num)=token.parse::<i32>(){
            s.push(num)
        }else{
            let right = s.pop().unwrap();
            let left = s.pop().unwrap();
            s.push(match token.as_str() {
                "+"=>{left+right},
                "-"=>{left-right},
                "*"=>{left*right},
                "/"=>{left/right},
                what=>{panic!("{what}")},
            })
        }
    }
    s.pop().unwrap()
}

fn main() {
    println!("Hello, world!");
}
