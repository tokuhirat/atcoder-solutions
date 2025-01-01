use proconio::{fastout, input};

const C: i64 = 1_100_000_000_000_000_000;
#[fastout]
fn main() {
    input! {
        (a, m, l, r): (i64, i64, i64, i64),
    };
    let s = C - C % m;
    let a = a % m;
    let l = l + s - a - 1;
    let r = r + s - a;
    let ans = r / m - l / m;
    println!("{}", ans);
}
