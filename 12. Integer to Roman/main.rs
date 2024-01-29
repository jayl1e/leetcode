struct Solution;
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut rt = String::new();
        let mapping = [
            (1000,"M"),(900,"CM"),(500,"D"),(400,"CD"),
            (100,"C"),(90,"XC"),(50,"L"),(40,"XL"),
            (10,"X"),(9,"IX"),(5,"V"),(4,"IV"),
            (1,"I")
        ];
        for (n,r) in mapping{
            while num>=n {
                rt.push_str(r);
                num -= n;
            }
        }
        rt
    }
}

fn main(){
    let n = 987;
    println!("{}: {}",n, Solution::int_to_roman(n));
}