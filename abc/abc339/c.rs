use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut m = 0;
    let mut s = 0;
    for &ai in &a {
        s += ai;
        m = m.min(s);
    }
    println!("{}", s - m);
}
