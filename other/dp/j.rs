use proconio::{fastout, input};

fn rec(dp: &mut [Vec<Vec<Option<f64>>>], n0: usize, n1: usize, n2: usize, n3: usize) -> f64 {
    if let Some(v) = dp[n1][n2][n3] {
        return v;
    }
    let mut v = n0 as f64;
    if n3 > 0 {
        v += (rec(dp, n0, n1, n2 + 1, n3 - 1) + 1.0) * n3 as f64;
    }
    if n2 > 0 {
        v += (rec(dp, n0, n1 + 1, n2 - 1, n3) + 1.0) * n2 as f64;
    }
    if n1 > 0 {
        v += (rec(dp, n0 + 1, n1 - 1, n2, n3) + 1.0) * n1 as f64;
    }
    v /= (n1 + n2 + n3) as f64;
    dp[n1][n2][n3] = Some(v);
    v
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = [0, 0, 0, 0];
    a.iter().for_each(|&ai| count[ai] += 1);

    let mut dp = vec![vec![vec![None; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = Some(0.0);
    let ans = rec(&mut dp, count[0], count[1], count[2], count[3]);
    println!("{}", ans);
}
