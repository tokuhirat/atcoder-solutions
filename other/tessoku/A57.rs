use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
        a: [usize; n]
    );

    let size = 29;
    let mut dp = vec![vec![0; n]; size + 1];
    for (i, &ai) in a.iter().enumerate() {
        dp[0][i] = ai - 1;
    }

    for j in 0..size {
        for i in 0..n {
            dp[j + 1][i] = dp[j][dp[j][i]];
        }
    }

    for _ in 0..q {
        input!(
            x: usize,
            y: usize
        );
        let mut cur = x - 1;

        for (i, dpi) in dp.iter().enumerate() {
            if (y >> i) & 1 == 1 {
                cur = dpi[cur];
            }
        }
        println!("{}", cur + 1);
    }
}
