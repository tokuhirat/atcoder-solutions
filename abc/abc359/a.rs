use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut ans = 0;
    for _ in 0..n {
        input!(
            s: String
        );
        if s == "Takahashi" {
            ans += 1;
        }
    }
    println!("{}", ans);
}
