use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        s: Chars,
    );
    let mut seen = HashSet::new();
    let mut ans = 0;
    for s in s.iter().permutations(n) {
        if seen.contains(&s) {
            continue;
        }
        seen.insert(s.clone());
        if !f(&s, n, k) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn f(s: &[&char], n: usize, k: usize) -> bool {
    for i in 0..n - k + 1 {
        let mut r = true;
        for j in 0..k {
            if s[i + j] != s[i + k - j - 1] {
                r = false;
                break;
            }
        }
        if r {
            return true;
        }
    }
    false
}
