struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|x|{x[1]});
        let mut cnt =1;
        let mut right = points[0][1];
        for p in points{
            if p[0]>right{
                right = p[1];
                cnt += 1;
            }
        }
        cnt
    }
}

fn main(){
    let bals = [[1,2],[2,3],[3,4],[4,5]];
    let vals = bals.into_iter().map(|x|{x.to_vec()}).collect();
    let rt = Solution::find_min_arrow_shots(vals);
    assert_eq!(2,rt);
}