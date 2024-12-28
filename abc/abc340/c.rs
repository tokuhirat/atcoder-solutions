use proconio::{fastout, input};
use std::collections::HashMap;

fn f(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 1 {
        return 0;
    }
    if let Some(&r) = memo.get(&n) {
        return r;
    }
    let r = f(n / 2, memo) + f((n + 1) / 2, memo) + n;
    memo.insert(n, r);
    r
}

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut memo = HashMap::new();
    println!("{}", f(n, &mut memo));
}
