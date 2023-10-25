fn main() {
    let mut nums = vec![1,2,3,4,3,2];
    let len1 = remove_element(&mut nums, 4);
    nums.truncate(len1 as usize);
    println!("got {len1}, {:?}", nums);
    let len2 = remove_element_filter(&mut nums, 3);
    println!("got {len2}, {:?}", nums);
}

static mut s1: i32 =3;

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