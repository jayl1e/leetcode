struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix[0].len();
        let length = matrix.len()*n;
        let mut left = 0;
        let mut right = length-1;
        while left<=right {
            let mid = (left+right)/2;
            let (x,y) = (mid/n, mid%n);
            let mv = matrix[x][y];
            if mv == target{
                return true;
            }else if mv < target{
                left = mid +1;
            }else{
                if mid == 0{
                    return false;
                }
                right = mid - 1;
            }
        }
        false
    }
}




fn main(){

}