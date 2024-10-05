use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n]
    );

    let mut idx = 0;
    let mut dp = vec![1 << 20; n];

    for &ai in a.iter() {
        let pos = dp.partition_point(|&x| x < ai);
        dp[pos] = ai;
        idx = idx.max(pos);
    }
    println!("{}", idx + 1);
}
