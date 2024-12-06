use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );
    for (i, &hi) in h.iter().enumerate().skip(1) {
        if hi > h[0] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
