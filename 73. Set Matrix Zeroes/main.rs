struct Solution;
impl Solution {
    pub fn set_zeroes1(matrix: &mut Vec<Vec<i32>>) {
        let mut buffer = matrix.clone();
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m{
            for j in 0..n{
                if matrix[i][j] == 0{
                    for y in 0..n{
                        buffer[i][y] = 0;
                    }
                    for x in 0..m{
                        buffer[x][j] = 0;
                    }
                }
            }
        }
        *matrix = buffer;
    }
    pub fn set_zeroes2(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut zero_row:Vec<bool> = vec![false;m];
        let mut zero_col:Vec<bool> = vec![false;n];
        for i in 0..m{
            for j in 0..n{
                if matrix[i][j] == 0{
                    zero_row[i] = true;
                    zero_col[j] = true;
                    

                }
            }
        }
        for (x, zero) in zero_row.into_iter().enumerate(){
            if zero{
                for y in 0..n{
                    matrix[x][y] = 0;
                }
            }
        }
        for (y, zero) in zero_col.into_iter().enumerate(){
            if zero{
                for x in 0..m{
                    matrix[x][y] = 0;
                }
            }
        };
    }
    pub fn set_zeroes3(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut first_row = false;
        let mut first_col = false;


        for i in 0..m{
            for j in 0..n{
                if matrix[i][j] == 0{
                    if i==0{
                        first_row = true;
                    }
                    if j==0{
                        first_col = true;
                    }
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for y in 1..n{
            if matrix[0][y]==0{
                for x in 1..m{
                    matrix[x][y] = 0;
                }
            }
        }
        for x in 1..m{
            if matrix[x][0]==0{
                for y in 1..n{
                    matrix[x][y] = 0;
                }
            }
        }
        if first_row{
            for y in 0..n{
                matrix[0][y] = 0;
            }
        }
        if first_col{
            for x in 0..m{
                matrix[x][0] = 0;
            }
        }
    }
}

fn main(){

}