struct Solution;
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&mut nums);
        nums
    }

    fn merge_sort(xs: &mut [i32]){
        if xs.len()<=1{
            return;
        }
        let length = xs.len();
        let mid = length/2;
        Self::merge_sort(&mut xs[..mid]);
        Self::merge_sort(&mut xs[mid..]);
        let (mut left, mut right) = (0,mid);
        let mut pos = 0;
        let mut buf = vec![0;length];
        while left<mid && right<length {
            if xs[left] <= xs[right]{
                buf[pos]=xs[left];
                left+=1
            }else{
                buf[pos]=xs[right];
                right+=1
            }
            pos+=1;
        }
        while left<mid {
            buf[pos]=xs[left];
            left+=1;
            pos+=1;
        }
        while right<length {
            buf[pos]=xs[right];
            right+=1;
            pos+=1;
        }
        xs.copy_from_slice(&buf);
    }

    fn quick_sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }
        let pos = Self::split(nums);
        Self::quick_sort(&mut nums[..pos]);
        Self::quick_sort(&mut nums[pos + 1..]);
    }

    fn split(nums: &mut [i32]) -> usize {
        let sep = nums[0];
        let (mut left, mut right) = (0, nums.len() - 1);
        'outer: while left < right {
            while nums[right] >= sep {
                if left >= right {
                    break 'outer;
                } else {
                    right -= 1;
                }
            }
            nums[left] = nums[right];
            left += 1;
            while nums[left] <= sep {
                if left >= right {
                    break 'outer;
                } else {
                    left += 1;
                }
            }
            nums[right] = nums[left];
            right -= 1;
        }
        nums[left] = sep;
        left
    }
}

fn main() {
    let mut arr = vec![2; 10000];
    arr[0] = 3;
    /*
    let mid = Solution::split(&mut arr);
    println!("{:?}:{}", arr, mid);
    */
    let rt = Solution::sort_array(arr);
    println!("{:?}", rt);
}
