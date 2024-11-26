use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
        w: [usize; n],
    );
    let mut b = vec![vec![]; n + 1];
    for (&ai, &wi) in a.iter().zip(w.iter()) {
        b[ai].push(wi);
    }
    let mut ans = 0;
    for bi in b.iter_mut() {
        if bi.len() <= 1 {
            continue;
        }
        bi.sort_unstable();
        for bij in bi.iter().take(bi.len() - 1) {
            ans += bij
        }
    }
    println!("{}", ans);
}
