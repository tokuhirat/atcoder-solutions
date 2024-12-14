use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        abcde: [i32; 5],
    );
    let l = ['A', 'B', 'C', 'D', 'E'];
    let mut ans = vec![];
    for i in 1..(1 << 5) {
        let mut s = vec![];
        let mut t = 0;
        for (j, lj) in l.iter().enumerate() {
            if i & (1 << j) != 0 {
                s.push(lj);
                t += abcde[j];
            }
        }

        ans.push((s, Reverse(t)));
    }
    ans.sort_by_key(|e| e.0.clone());
    ans.sort_by_key(|e| e.1);
    for a in &ans {
        println!("{}", a.0.iter().join(""));
    }
}
