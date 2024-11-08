use itertools::Itertools;
use proconio::{fastout, input};

fn dist(a: (i64, i64), b: (i64, i64)) -> f64 {
    let d = ((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as f64;
    d.sqrt()
}

#[fastout]
fn main() {
    input!(
        (n, s, t): (usize, usize, usize),
        abcd: [(i64, i64, i64, i64); n],
    );

    let mut ans = f64::MAX;

    for pi in (0..n).permutations(n) {
        for i in 0..1 << n {
            let mut s = 0.;
            let mut current = (0, 0);
            for j in 0..n {
                if i & 1 << j != 0 {
                    s += dist(current, (abcd[pi[j]].0, abcd[pi[j]].1));
                    current = (abcd[pi[j]].2, abcd[pi[j]].3);
                } else {
                    s += dist(current, (abcd[pi[j]].2, abcd[pi[j]].3));
                    current = (abcd[pi[j]].0, abcd[pi[j]].1);
                }
            }
            ans = ans.min(s);
        }
    }
    ans /= s as f64;
    let constants = abcd
        .iter()
        .fold(0., |acc, x| acc + dist((x.0, x.1), (x.2, x.3)));
    ans += constants / t as f64;
    println!("{}", ans);
}
