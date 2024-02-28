struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut q:VecDeque<&String> = VecDeque::new();
        let mut visited:Vec<bool> = vec![false;bank.len()];
        q.push_back(&start_gene);
        let mut level = 0;
        while !q.is_empty() {
            for _ in 0..q.len(){
                let cur = q.pop_front().unwrap();
                if *cur == end_gene{
                    return level;
                }
                for idx in 0..visited.len(){
                    if visited[idx]{
                        continue;
                    }
                    if Self::conjunc(cur, &bank[idx]){
                        visited[idx] = true;
                        q.push_back(&bank[idx])
                    }
                }
            }
            level += 1;
        }
        -1
    }

    fn conjunc(left: &String, right: &String)->bool{
        left.bytes().zip(right.bytes()).filter(|(x,y)|{*x!=*y}).count() == 1
    }
}

fn main(){
    let startGene = "AACCGGTT";
    let endGene = "AAACGGTA";
    let bank = ["AACCGGTA","AACCGCTA","AAACGGTA"];
    let step = Solution::min_mutation(startGene.to_string(), endGene.to_string(), bank.iter().map(|x|{x.to_string()}).collect());
    println!("step is {step}")
}