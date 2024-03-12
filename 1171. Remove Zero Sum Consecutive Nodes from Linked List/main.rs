// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        const DELETED: i32 = 1001;
        let mut xs: Vec<i32> = Vec::new();
        while let Some(val) = head {
            xs.push(val.val);
            head = val.next;
        }
        for i in 0..xs.len() {
            let mut s = 0;
            for j in (0..=i).rev() {
                if xs[j] != DELETED {
                    s += xs[j];
                    if s == 0 {
                        for k in j..=i {
                            xs[k] = DELETED;
                        }
                        break;
                    }
                }
            }
        }
        for val in xs.into_iter().rev() {
            if val != DELETED {
                head = Some(Box::new(ListNode { val, next: head }))
            }
        }
        head
    }
}

fn vec2list(xs: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for val in xs.into_iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }))
    }
    head
}

fn list2vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut xs = vec![];
    while let Some(node) = head {
        xs.push(node.val);
        head = node.next;
    }
    xs
}

fn main() {
    let h = vec2list(vec![1, 2, -3, 3, 1]);
    let h = Solution::remove_zero_sum_sublists(h);
    assert_eq!(vec![3, 1], list2vec(h));
}
