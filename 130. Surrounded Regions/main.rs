struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        if m==0 || n==0{
            return;
        }

        for j in 0..n{
            Self::dfs(board, 0, j as isize);
            Self::dfs(board, (m-1) as isize, j as isize);
        }
        for i in 0..m{
            Self::dfs( board, i as isize, 0);
            Self::dfs( board, i as isize, (n-1) as isize);
        }
        for i in 0..m{
            for j in 0..n{
                if board[i][j] == 'O'{
                    board[i][j] = 'X'
                }else if board[i][j] == 'Q'{
                    board[i][j] = 'O'
                }
            }
        }
    }
    fn dfs(board: &mut Vec<Vec<char>>, i:isize, j:isize){
        if i<0 || i>=board.len() as isize || j<0 || j>=board[0].len() as isize {
            return;
        }
        let ui = i as usize;
        let uj = j as usize;
        if board[ui][uj] == 'X' || board[ui][uj] == 'Q'{
            return;
        }
        board[ui][uj] = 'Q';
        for (di,dj) in [(-1,0),(1,0),(0,-1),(0,1)]{
            Self::dfs(board, i+di, j+dj)
        }
    }
}

fn main() {
    let mut q = vec![vec!['X','X','X','X'],vec!['X','X','O','X'],vec!['X','O','X','X']];
    Solution::solve(&mut q);
    println!("{:?}", q);
}