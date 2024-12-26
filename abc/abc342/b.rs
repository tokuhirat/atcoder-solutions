use proconio::{fastout, input};

fn solv(p: &[usize], a: usize, b: usize) {
    for &pi in p {
        if pi == a {
            println!("{}", a);
            return;
        }
        if pi == b {
            println!("{}", b);
            return;
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
    };

    for _ in 0..q {
        input! {(a, b): (usize, usize)};
        solv(&p, a, b);
    }
}
