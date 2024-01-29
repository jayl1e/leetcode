
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let k = Self::find_k(&nums,0,nums.len()-1) as isize;
        match Self::bin_search(&nums, target, 0, k-1){
            Some(idx)=>{
                idx as i32
            },
            None =>{
                match Self::bin_search(&nums, target, k, (nums.len()-1) as isize){
                    Some(idx)=>{
                        idx as i32
                    },
                    None=>{
                        -1
                    }
                }
            }
        }
    }
    fn find_k(nums: &Vec<i32>, left:usize, right:usize)->usize{
        if right-left<2{
            if nums[left]>nums[right]{
                return right;
            }
            return 0;
        };
        let mid = (left+right)/2;
        if nums[mid]>=nums[right]{
            return Self::find_k(nums, mid, right);
        }
        if nums[mid] <= nums[left]{
            return Self::find_k(nums, left, mid);
        }
        match Self::find_k(nums, left, mid){
            0=>Self::find_k(nums, mid, right),
            x=>x
        }
    }
    fn bin_search(nums: &Vec<i32>, target: i32, left:isize, right:isize)->Option<usize>{
        if right < left{
            return None;
        }else if right==left{
            if nums[left as usize] == target {
                return Some(left as usize);
            }else{
                return None;
            }
        }
        let mid = (left+right)/2;
        if nums[mid as usize]==target{
            Some(mid as usize)
        }else if nums[mid as usize]<target{
            Self::bin_search(nums, target, mid+1, right)
        }else{
            Self::bin_search(nums, target, left, mid-1)
        }
    }
}

fn main(){
    let nums = vec![1,3];
    let k = Solution::find_k(&nums, 0, nums.len()-1);
    println!("k is {k}");
    let target = 0;
    let result = Solution::search(nums, target);
    println!("{}", result);
}