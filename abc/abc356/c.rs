use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        (n, m, k): (usize, usize, usize),
    );
    let mut c = Vec::with_capacity(m);
    let mut a = Vec::with_capacity(m);
    let mut r = Vec::with_capacity(m);
    for _ in 0..m {
        input!(ci: usize);
        c.push(ci);
        input!(ai: [Usize1; ci]);
        let mut ai_h = HashMap::new();
        for aij in ai {
            let entry = ai_h.entry(aij).or_insert(0);
            *entry += 1;
        }
        a.push(ai_h);
        input!(ri: char);
        r.push(ri);
    }
    let mut ans = 0;
    for pat in 0..1 << n {
        let mut b = true;
        for i in 0..m {
            let mut cnt = 0;
            for j in 0..n {
                if pat >> j & 1 != 0 {
                    let entry = a[i].entry(j).or_insert(0);
                    cnt += *entry;
                }
            }

            b &= (cnt < k && r[i] == 'x') || (cnt >= k && r[i] == 'o');
        }
        if b {
            ans += 1;
        }
    }
    println!("{}", ans);
}
