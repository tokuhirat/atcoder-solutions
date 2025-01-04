use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    };

    let mut ans = 0;
    for i in 0..=9 {
        for j in 0..=9 {
            if i * j != x {
                ans += i * j;
            }
        }
    }
    println!("{}", ans);
}
