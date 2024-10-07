use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut lr: [(usize, usize); n],
    );

    lr.sort_unstable_by_key(|k| k.1);
    let mut time = 0;
    let mut ans = 0;
    for &(l, r) in lr.iter() {
        if l >= time {
            ans += 1;
            time = r;
        }
    }
    println!("{}", ans);
}
