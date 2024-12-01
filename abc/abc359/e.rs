use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );
    let mut v = Vec::with_capacity(n);

    let mut ans = Vec::with_capacity(n);

    for (i, &hi) in h.iter().enumerate() {
        while v.last().unwrap_or(&(usize::MAX, 0, 0)).0 < hi {
            v.pop();
        }
        if v.is_empty() {
            v.push((hi, i, hi * (i + 1)));
        } else {
            let (_, last_i, last_v) = v.last().unwrap();
            v.push((hi, i, hi * (i - last_i) + last_v));
        }
        ans.push(v.last().unwrap().2 + 1);
    }

    println!("{}", ans.iter().join(" "));
}
