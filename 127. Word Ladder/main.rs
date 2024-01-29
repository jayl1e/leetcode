use std::collections::{VecDeque, HashSet};
struct S1;
impl S1{
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_list=word_list;
        word_list.insert(0, begin_word);
        let mut adjm = vec![vec![false;word_list.len()];word_list.len()];
        let mut target = 0;
        for i in 0..adjm.len(){
            for j in 0..adjm.len(){
                adjm[i][j] = Self::is_adja(&word_list[i], &word_list[j]);
            }
            if target==0 && word_list[i] == end_word{
                target = i;
            }
        }

        let mut q = VecDeque::new();
        let mut visited = vec![false;word_list.len()];
        let mut dist =1;
        q.push_back(0);
        visited[0] = true;

        while !q.is_empty() {
            dist += 1;
            for _i in 0..q.len(){
                let p = q.pop_front().unwrap();
                for (i,v) in adjm[p].iter().enumerate(){
                    if *v && !visited[i]{
                        q.push_back(i);
                        visited[i]=true;
                        if i == target{
                            return dist;
                        }
                    }
                }
            }
        }
        0
    }
    
    fn is_adja(left:&str, right:&str)->bool{
        left.chars().zip(right.chars()).map(|(l,r)|{(l!=r)as isize}).sum::<isize>()==1
    }
}

struct  S2;
impl S2 {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words : HashSet<String> = word_list.into_iter().collect();
        if!words.contains(&end_word){
            return 0;
        }
        let mut q = VecDeque::new();
        let mut dist = 1;
        words.remove(&begin_word);
        q.push_back(begin_word);
        while !q.is_empty() {
            dist += 1;
            for _ in 0..q.len(){
                let ow = q.pop_front().unwrap();
                for i in 0..ow.len(){
                    let mut o = ow.clone();
                    for nc in b'a'..=b'z'{
                        unsafe{
                            o.as_bytes_mut()[i]=nc;
                        }
                        
                        if words.contains(&o){
                            words.remove(&o);
                            if o == end_word{
                                return dist;
                            }
                            q.push_back(o.clone());
                        }
                    }
                }
            }
        }
        0
    }
}

fn main(){
    let words = vec!["hot","dog","dot"];
	let d=S2::ladder_length("hot".to_string(), "dog".to_string(), words.into_iter().map(|x|{x.to_string()}).collect());
	println!("{}",d);
}
