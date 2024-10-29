use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        n: usize,
        x: usize,
        mut a: Chars
    );

    let mut queue = VecDeque::new();
    queue.push_back(x - 1);
    a[x - 1] = '@';

    while let Some(pos) = queue.pop_front() {
        if pos > 0 && a[pos - 1] == '.' {
            a[pos - 1] = '@';
            queue.push_back(pos - 1);
        }
        if pos < n - 1 && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            queue.push_back(pos + 1);
        }
    }

    for ai in a.iter() {
        print!("{}", ai);
    }
    println!("");
}
