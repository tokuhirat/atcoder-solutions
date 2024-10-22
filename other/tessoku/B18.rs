use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: usize,
        a: [usize; n],
    );

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            dp[i][j] = dp[i - 1][j];
            if a[i - 1] <= j && j - a[i - 1] <= s && dp[i - 1][j - a[i - 1]] {
                dp[i][j] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    let mut i = n;
    let mut v = s;
    while i > 0 {
        if !dp[i - 1][v] {
            ans.push(i);
            v -= a[i - 1];
        }
        i -= 1;
    }

    println!("{}", ans.len());
    for (i, &ai) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ai);
    }
    println!("");
}
