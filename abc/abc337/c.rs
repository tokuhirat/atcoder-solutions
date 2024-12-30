use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };
    let mut map = HashMap::new();
    for (i, &ai) in a.iter().enumerate() {
        map.insert(ai, i as i32 + 1);
    }
    let mut now = -1;
    for i in 0..n {
        now = *map.get(&now).unwrap();
        if i > 0 {
            print!(" ");
        }
        print!("{}", now);
    }
    println!("");
}
