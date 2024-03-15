struct Solution;

impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let zeros = nums.iter().filter(|&&item|{item==0}).count();
        match zeros {
            0=>{
                let prod:i32 = nums.iter().product();
                nums.iter_mut().for_each(|item|{*item = prod/(*item)});
            },
            1=>{
                let prod:i32 = nums.iter().filter(|&&item|{item!=0}).product();
                nums.iter_mut().for_each(|item|{*item = if *item == 0{prod}else{0}});
            },
            _=>{
                nums.iter_mut().for_each(|item|{*item = 0});
            }
        }
        nums
    }
}

fn main(){
    let rt =Solution::product_except_self(vec![1,2,3,0]);
    println!("{:?}", rt);
}