use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [i32; n - 1],
        b: [i32; n - 2]
    );

    let mut dp = vec![(i32::MAX, usize::MAX); n];
    dp[0].0 = 0;
    dp[1].0 = a[0];
    dp[1].1 = 0;

    for i in 2..n {
        if dp[i - 2].0 + b[i - 2] < dp[i - 1].0 + a[i - 1] {
            dp[i].0 = dp[i - 2].0 + b[i - 2];
            dp[i].1 = i - 2;
        } else {
            dp[i].0 = dp[i - 1].0 + a[i - 1];
            dp[i].1 = i - 1;
        }
    }

    let mut ans = vec![];
    let mut current = n - 1;
    while current > 0 {
        ans.push(current + 1);
        current = dp[current].1;
    }
    ans.push(1);
    ans.reverse();
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
