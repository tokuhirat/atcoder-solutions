use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        tv: [(usize, usize); n],
    );

    let mut s: usize = 0;
    let mut i = 0;
    for t in 1..=tv[n - 1].0 {
        s = s.saturating_sub(1);
        if t == tv[i].0 {
            s += tv[i].1;
            i += 1;
        }
    }
    println!("{}", s);
}
