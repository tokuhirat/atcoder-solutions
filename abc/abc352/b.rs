use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
        t: Chars,
    );
    let mut ans = Vec::with_capacity(s.len());
    let mut j = 0;
    for &si in &s {
        while si != t[j] {
            j += 1;
        }
        j += 1;
        ans.push(j);
    }
    println!("{}", ans.iter().join(" "));
}
