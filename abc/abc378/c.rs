use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let mut b = Vec::with_capacity(n);
    let mut cnt = HashMap::new();
    for (i, ai) in a.iter().enumerate() {
        let v = cnt.entry(ai).or_insert(-1);
        b.push(*v);
        cnt.insert(ai, (i + 1) as i32);
    }

    for (i, &bi) in b.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", bi)
    }
    println!("");
}
