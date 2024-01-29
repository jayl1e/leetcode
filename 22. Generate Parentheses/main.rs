struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut rt:Vec<String> = vec![];
        let mut buffer = vec!['(';2*n as usize];
        Self::gen(0,0,n,&mut buffer, &mut rt);
        rt
    }
    fn gen(filled:usize, cum:isize, n:i32, buffer: &mut Vec<char>, rt:&mut Vec<String>){
        if filled==buffer.len(){
            rt.push(buffer.iter().collect())
        }
        if n>0{
            buffer[filled]='(';
            Self::gen(filled+1, cum+1,n-1,buffer, rt);
        }
        if cum>0{
            buffer[filled]=')';
            Self::gen(filled+1, cum-1,n,buffer, rt);
        }
    }
}
fn main(){
    let ps = Solution::generate_parenthesis(3);
    println!("{:?}",ps);
}