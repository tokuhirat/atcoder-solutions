use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n]
    );

    let mut map = HashMap::new();

    for ai in a.iter() {
        let cnt = map.entry(ai).or_insert(0_usize);
        *cnt += 1;
    }
    let mut ans = 0;
    for cnt in map.into_values() {
        if cnt >= 2 {
            ans += cnt * (cnt - 1) / 2;
        }
    }
    println!("{}", ans);
}
