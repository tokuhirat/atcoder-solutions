use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        s: Chars
    );

    let mut ans = vec![];
    let mut que = VecDeque::new();
    for (i, &si) in s.iter().enumerate() {
        if si == '(' {
            que.push_back(i);
        } else {
            let start = que.pop_back().unwrap();
            ans.push((start + 1, i + 1));
        }
    }
    for ansi in ans.iter() {
        println!("{} {}", ansi.0, ansi.1);
    }
}
