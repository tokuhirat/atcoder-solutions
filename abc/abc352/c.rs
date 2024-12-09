use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut ab: [(usize, usize); n],
    );
    ab.sort_unstable_by_key(|x| x.1 - x.0);
    let mut ans = 0;
    for &(a, _) in &ab {
        ans += a;
    }
    ans += ab[n - 1].1 - ab[n - 1].0;
    println!("{}", ans);
}
