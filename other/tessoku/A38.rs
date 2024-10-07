use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    );
    let mut maxtime = vec![24; d];
    for &(l, r, h) in lrh.iter() {
        for time in maxtime.iter_mut().take(r).skip(l - 1) {
            *time = (*time).min(h);
        }
    }
    let ans: usize = maxtime.iter().sum();
    println!("{}", ans);
}
