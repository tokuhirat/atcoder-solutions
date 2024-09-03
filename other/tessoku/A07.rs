use proconio::input;
use std::collections::HashMap;
fn main() {
    input!(d: usize, n: usize);
    input!(lr: [(usize, usize); n]);

    let mut num = HashMap::new();
    for (l, r) in lr {
        let count = num.entry(l).or_insert(0);
        *count += 1;
        let count = num.entry(r + 1).or_insert(0);
        *count -= 1;
    }
    let mut sum = 0;
    for i in 1..=d {
        let count = num.entry(i).or_insert(0);
        sum += *count;
        println!("{}", sum);
    }
}
