use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        q: usize
    );

    let mut hash_map = HashMap::new();
    for _ in 0..q {
        input!(
            t: usize,
            x: String
        );
        if t == 1 {
            input!(y: usize);
            hash_map.insert(x, y);
        } else {
            println!("{}", hash_map.get(&x).unwrap());
        }
    }
}
