use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
    );
    let mut v = vec![vec![0; n + 1]; 30];
    for i in 0..=n {
        v[0][i] = i;
        for si in i.to_string().chars() {
            v[0][i] -= si as usize - '0' as usize;
        }
    }

    for i in 1..30 {
        for j in 0..=n {
            v[i][j] = v[i - 1][v[i - 1][j]];
        }
    }

    let mut ans = (0..=n).collect::<Vec<_>>();
    let mut nxt = (0..=n).collect::<Vec<_>>();
    for (i, vi) in v.iter().enumerate() {
        if k & 1 << i != 0 {
            for j in 0..=n {
                nxt[j] = vi[ans[j]];
            }
            swap(&mut ans, &mut nxt);
        }
    }

    for &ai in ans.iter().skip(1) {
        println!("{}", ai);
    }
}
