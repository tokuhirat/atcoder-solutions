use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        n: Usize1,
    );

    let mut cnt = 0;
    for i in 0..20 {
        let nxt = 10_usize.pow(i + 1) - 10_usize.pow(i);
        if cnt + nxt >= n {
            let num = 10_usize.pow(i) + n - cnt - 1;
            let num: Vec<_> = num.to_string().chars().collect();
            let len = num.len() * 2 - 1;
            let mut ans = vec!['1'; len];
            for (i, &ni) in num.iter().enumerate() {
                ans[i] = ni;
                ans[len - i - 1] = ni;
            }
            println!("{}", ans.into_iter().join(""));
            break;
        }
        if cnt + 2 * nxt >= n {
            let num = 10_usize.pow(i) + n - nxt - cnt - 1;
            let num: Vec<_> = num.to_string().chars().collect();
            let len = num.len() * 2;
            let mut ans = vec!['1'; len];
            for (i, &ni) in num.iter().enumerate() {
                ans[i] = ni;
                ans[len - i - 1] = ni;
            }
            println!("{}", ans.into_iter().join(""));
            break;
        }
        cnt += 2 * nxt;
    }
}
