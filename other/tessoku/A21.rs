use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut pa: [(usize, usize); n],
    );
    pa.insert(0, (0, 0));
    pa.push((0, 0));

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for len in (0..=n - 2).rev() {
        for l in 1..=n - len {
            let r = len + l;

            let mut s1 = 0;
            if l <= pa[l - 1].0 && pa[l - 1].0 <= r {
                s1 = pa[l - 1].1;
            }
            let mut s2 = 0;
            if l <= pa[r + 1].0 && pa[r + 1].0 <= r {
                s2 = pa[r + 1].1;
            }

            if l == 1 {
                dp[l][r] = dp[l][r + 1] + s2;
            } else if r == n {
                dp[l][r] = dp[l - 1][r] + s1;
            } else {
                dp[l][r] = (dp[l - 1][r] + s1).max(dp[l][r + 1] + s2);
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = ans.max(dp[i][i]);
    }
    println!("{}", ans);
}
