use proconio::{fastout, input, marker::Usize1};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        (_n, m): (usize, usize),
        xyc: [(Usize1, Usize1, char); m],
    };
    let mut b: Vec<_> = xyc
        .iter()
        .filter(|f| f.2 == 'B')
        .map(|f| (f.0, f.1))
        .collect();
    b.sort_unstable();
    b.reverse();
    let mut w: Vec<_> = xyc
        .iter()
        .filter(|f| f.2 == 'W')
        .map(|f| (f.0, f.1))
        .collect();
    w.sort_unstable();
    w.reverse();

    let mut bi = 0;
    let mut b_yx = BTreeSet::new();
    for &wi in &w {
        while bi < b.len() && b[bi].0 >= wi.0 {
            b_yx.insert((b[bi].1, b[bi].0));
            bi += 1;
        }
        if let Some(y) = b_yx.last() {
            if y.0 >= wi.1 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
