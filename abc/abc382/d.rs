use itertools::Itertools;
use proconio::{fastout, input};
use std::{collections::VecDeque, mem::swap};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
    );
    let mut queue = VecDeque::new();
    for i in 1..=(m - (n - 1) * 10) {
        queue.push_back(vec![i; 1]);
    }
    for i in 2..=n {
        let mut nxt = VecDeque::new();
        let max = m - (n - i) * 10;
        while let Some(cur) = queue.pop_front() {
            let last = cur.last().unwrap();
            for j in last + 10..=max {
                nxt.push_back([cur.clone(), vec![j; 1]].concat());
            }
        }
        swap(&mut queue, &mut nxt);
    }
    println!("{}", queue.len());
    for q in queue {
        println!("{}", q.iter().join(" "));
    }
}
