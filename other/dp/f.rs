use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (s, t): (Chars, Chars),
    };
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for (i, &si) in s.iter().enumerate() {
        for (j, &tj) in t.iter().enumerate() {
            dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            if si == tj {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
            }
        }
    }

    let mut ans = vec![];
    let mut i = s.len();
    let mut j = t.len();
    while ans.len() < dp[s.len()][t.len()] {
        while dp[i][j] == dp[i - 1][j] {
            i -= 1;
        }
        while dp[i][j] == dp[i][j - 1] {
            j -= 1;
        }
        i -= 1;
        j -= 1;
        ans.push(t[j]);
    }

    if !ans.is_empty() {
        println!("{}", ans.iter().rev().join(""));
    }
}
