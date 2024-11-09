use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        n: usize,
        qr: [(usize, usize); n],
        q: usize,

    );

    for _ in 0..q {
        input!(
            (t, d): (Usize1, usize)
        );
        let (q, r) = qr[t];
        let ans = if d % q <= r {
            d + r - d % q
        } else {
            d + q + r - d % q
        };
        println!("{}", ans);
    }
}
