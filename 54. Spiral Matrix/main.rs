struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rt:Vec<i32> = Vec::with_capacity(matrix.len()*matrix[0].len());
        let (mut left, mut right,mut  top,mut bottom) = (0,matrix[0].len(),0,matrix.len());
        let mut order = 0;
        while left<right && top<bottom {
          match order {
            0 => {
                for idx in left..right{
                    rt.push(matrix[top][idx])
                }
                top += 1;
            },
            1 => {
                for idx in top..bottom{
                    rt.push(matrix[idx][right-1])
                }
                right -= 1; 
            },
            2 => {
                for idx in (left..right).rev(){
                    rt.push(matrix[bottom-1][idx])
                }
                bottom -= 1;
            },
            3 => {
                for idx in (top..bottom).rev(){
                    rt.push(matrix[idx][left])
                }
                left += 1; 
            },
            _ => panic!()
          }
          order = (order+1)%4
        }
        rt
    }
}

fn main(){
    let mat = [[1,2,3],[4,5,6],[7,8,9]];
    let mat = mat.iter().map(|x|{x.to_vec()}).collect();
    let rt = Solution::spiral_order(mat);
    assert_eq!(vec![1,2,3,6,9,8,7,4,5], rt);
}