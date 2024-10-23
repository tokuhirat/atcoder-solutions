use proconio::{fastout, input};

fn dist(p1: (usize, usize), p2: (usize, usize)) -> f32 {
    let x_diff = p1.0.abs_diff(p2.0);
    let y_diff = p1.1.abs_diff(p2.1);
    ((x_diff * x_diff + y_diff * y_diff) as f32).sqrt()
}

#[fastout]
fn main() {
    input!(
        n: usize,
        xy: [(usize, usize); n],
    );

    let mut dp = vec![vec![f32::MAX; n]; 1 << n];

    dp[0][0] = 0.;
    for bit in 0..1 << n {
        for cur in 0..n {
            if dp[bit][cur] == f32::MAX {
                continue;
            }
            for nxt in 0..n {
                if bit & (1 << nxt) != 0 {
                    continue;
                }
                let d = dist(xy[cur], xy[nxt]);
                dp[bit | (1 << nxt)][nxt] = dp[bit | (1 << nxt)][nxt].min(dp[bit][cur] + d);
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
}
