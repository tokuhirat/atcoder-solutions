use proconio::{fastout, input, marker::Usize1};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!(
        (h, w, q): (usize, usize, usize),
    );

    let mut row = vec![(0..w).collect::<BTreeSet<_>>(); h];
    let mut col = vec![(0..h).collect::<BTreeSet<_>>(); w];

    for _ in 0..q {
        input!(
            (r, c): (Usize1, Usize1),
        );

        if row[r].contains(&c) {
            row[r].remove(&c);
            col[c].remove(&r);
            continue;
        }

        if let Some(&left) = row[r].range(..c).last() {
            row[r].remove(&left);
            col[left].remove(&r);
        }
        if let Some(&right) = row[r].range(c..).next() {
            row[r].remove(&right);
            col[right].remove(&r);
        }
        if let Some(&left) = col[c].range(..r).last() {
            row[left].remove(&c);
            col[c].remove(&left);
        }
        if let Some(&right) = col[c].range(r..).next() {
            row[right].remove(&c);
            col[c].remove(&right);
        }
    }
    let mut ans = 0;
    for r in &row {
        ans += r.len();
    }
    println!("{}", ans);
}
