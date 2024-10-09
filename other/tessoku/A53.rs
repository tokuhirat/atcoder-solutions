use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input!(
        q: usize
    );

    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input!(t: usize);
        if t == 1 {
            input!(x: i32);
            heap.push(-x);
        } else if t == 2 {
            println!("{}", -heap.peek().unwrap());
        } else {
            heap.pop();
        }
    }
}
