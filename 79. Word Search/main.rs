
struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut visited : Vec<Vec<bool>> = vec![vec![false;board[0].len()];board.len()];
        let words:Vec<char> = word.chars().collect();
        let m=board.len();
        let n = board[0].len();
        for i in 0..m{
            for j in 0..n{
                if Self::exist_rc(i,j,&board, &words,&mut visited,m,n){
                    return true;
                }
            }
        };
        false
    }

    fn exist_rc(x:usize, y:usize,board:&Vec<Vec<char>>, word:&[char], visited:&mut Vec<Vec<bool>>, m:usize,n:usize)->bool{
        if word.is_empty(){
            return true;
        }
        if x==usize::MAX || x==m || y == usize::MAX ||y==n || visited[x][y]{
            return false;
        }
        if board[x][y] == word[0]{
            visited[x][y] = true;
            let next = &word[1..];
            for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)]{
                if Self::exist_rc(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy), board, next, visited,m,n){
                    return true;
                }
            }
            visited[x][y] = false;
        }
        false
    }
}
fn main(){

}