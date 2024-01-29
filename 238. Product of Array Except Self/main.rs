use std::vec::Vec;
struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let (prod, zeros) = nums.iter().fold((1,0), |(mut prod, mut zeros),item|{
            if *item==0{
                if zeros!=0{
                    prod = 0
                }
                zeros+=1;
            }else{
                prod *= item;
            }
            (prod,zeros)
        });
        match zeros{
            0=>{
                nums.iter_mut().for_each(|item|{
                    *item = prod/(*item);
                })
            },
            1=>{
                nums.iter_mut().for_each(|item|{
                    if *item==0{
                        *item = prod;
                    }else{
                        *item = 0;
                    }
                })
            },
            _=>{
                nums.iter_mut().for_each(|item|{
                    *item =0;
                })
            }
        }
        nums
    }
}

fn main(){
    let rt =Solution::product_except_self(vec![1,2,3,0]);
    println!("{:?}", rt);
}