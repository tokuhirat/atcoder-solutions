use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [i32; n - 1],
    );
    let ans: i32 = a.iter().sum();
    println!("{}", ans.saturating_neg());
}
