struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut rt = 0;
        let  mapping = [
            ("M",1000),("CM",900),("D",500), ("CD",400),
            ("C",100),("XC",90),("L",50), ("XL",40),
            ("X",10),("IX",9),("V",5), ("IV",4),
            ("I",1)
        ];
        let mut s:&str = &s;
        for (r,a) in mapping{
            while let Some(t) = s.strip_prefix(r) {
                rt += a;
                s = t;
            }
        }
        rt
    }
}

fn main(){
    let roman = "LVIII";
    let integer = Solution::roman_to_int(roman.to_owned());
    println!("{}: {}", roman,integer)
}