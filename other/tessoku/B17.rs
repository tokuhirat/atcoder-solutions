use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );

    let mut dp = vec![usize::MAX; n + 1];
    dp[1] = 0;
    dp[2] = h[0].abs_diff(h[1]);
    for i in 3..=n {
        let cost1 = dp[i - 1] + h[i - 1].abs_diff(h[i - 2]);
        let cost2 = dp[i - 2] + h[i - 1].abs_diff(h[i - 3]);
        dp[i] = cost1.min(cost2);
    }

    let mut route = vec![n];
    let mut i = n;
    while i > 1 {
        if dp[i] >= dp[i - 1] && dp[i] - dp[i - 1] == h[i - 1].abs_diff(h[i - 2]) {
            i -= 1;
        } else {
            i -= 2;
        }
        route.push(i);
    }
    route.reverse();
    println!("{}", route.len());
    for (i, &r) in route.iter().enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", r)
    }
    println!("");
}
