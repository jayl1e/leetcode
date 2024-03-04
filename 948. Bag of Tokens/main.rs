struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort();
        let mut left = 0;
        let mut right = tokens.len();
        let mut score = 0;
        while left<right {
            if tokens[left]<=power{
                power -= tokens[left];
                score += 1;
                left += 1;
            }else if score>0 && right>left+1{
                power += tokens[right-1];
                score -= 1;
                right -= 1;
            }else{
                break;
            }
        }
        score
    }
}

fn main(){
    let tokens = vec![100,200,300,400];
    let power = 200;
    let score = Solution::bag_of_tokens_score(tokens, power);
    assert_eq!(2, score);
}