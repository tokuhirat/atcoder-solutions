use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut xy: [(i32, i32); n],
    );

    for (_, yi) in xy.iter_mut() {
        *yi *= -1
    }
    xy.sort_unstable();

    let mut dp = vec![i32::MAX; n];
    let mut ans = 0;
    for &(_, yi) in xy.iter() {
        let pos = dp.partition_point(|&e| e < -yi);
        dp[pos] = -yi;
        ans = ans.max(pos);
    }

    println!("{}", ans + 1);
}
