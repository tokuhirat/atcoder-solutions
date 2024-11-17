use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars
    );
    let mut dp = Vec::with_capacity(n + 1);
    dp.push((0, 0, 0));
    for &si in &s {
        let &(r, p, s) = dp.last().unwrap();
        match si {
            'R' => {
                let nxt_r = p.max(s);
                let nxt_p = r.max(s) + 1;
                let nxt_s = 0;
                dp.push((nxt_r, nxt_p, nxt_s));
            }
            'P' => {
                let nxt_r = 0;
                let nxt_p = s.max(r);
                let nxt_s = p.max(r) + 1;
                dp.push((nxt_r, nxt_p, nxt_s));
            }
            'S' => {
                let nxt_r = s.max(p) + 1;
                let nxt_p = 0;
                let nxt_s = r.max(p);
                dp.push((nxt_r, nxt_p, nxt_s));
            }
            _ => unreachable!(),
        };
    }
    let ans = dp[n].0.max(dp[n].1).max(dp[n].2);
    println!("{}", ans);
}
