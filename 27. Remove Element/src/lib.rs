#[no_mangle]
pub extern "C" fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut filled = 0;
    for i in 0..nums.len(){
        if nums[i] != val{
            nums[filled] = nums[i];
            filled += 1;
        }
    }
    filled as i32
}

#[no_mangle]
pub fn remove_element_filter(nums: &mut Vec<i32>, val:i32) -> i32{
    nums.retain(|&x:&i32|{x!=val});
    nums.len() as i32
}