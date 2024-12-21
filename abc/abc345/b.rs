use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        x: i64,
    );
    if x >= 0 {
        let ans = (x + 9) / 10;
        println!("{}", ans);
    } else {
        let ans = (-x) / 10;
        println!("{}", -ans);
    }
}
