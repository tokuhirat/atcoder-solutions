use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = vec![];
    let mut ans = 0;
    for &ai in &a {
        if ai == 0 {
            ans += 1;
        } else {
            b.push(ai);
        }
    }
    let res = n - ans;
    ans = n * (n - 1) / 2 - res * (res - 1) / 2;

    let mut map = HashMap::new();
    for &bi in &b {
        let mut i = 1;
        let mut d = 0;
        while i * i <= bi {
            if bi % (i * i) == 0 {
                d = bi / (i * i);
            }
            i += 1;
        }
        *map.entry(d).or_insert(0) += 1;
    }
    for (_, &cnt) in map.iter() {
        ans += cnt * (cnt - 1) / 2;
    }

    println!("{}", ans);
}
