use proconio::{fastout, input, marker::Usize1};
use std::collections::{BTreeMap, HashSet};

#[fastout]
fn main() {
    input!(
        (h, w, m): (usize, usize, usize),
        tax: [(usize, Usize1, usize); m],
    );
    let mut c = BTreeMap::new();
    let mut row = HashSet::new();
    let mut col = HashSet::new();
    for &(t, a, x) in tax.iter().rev() {
        match t {
            1 => {
                if row.contains(&a) {
                    continue;
                }
                row.insert(a);
                if col.len() < w {
                    let e = c.entry(x).or_insert(0);
                    *e += w - col.len();
                }
            }
            2 => {
                if col.contains(&a) {
                    continue;
                }
                col.insert(a);
                if row.len() < h {
                    let e = c.entry(x).or_insert(0);
                    *e += h - row.len();
                }
            }
            _ => unreachable!(),
        }
    }

    let s: usize = c.values().sum();
    let s0 = h * w - s;
    if s0 > 0 {
        let e = c.entry(0).or_insert(0);
        *e += s0;
    }
    let c: Vec<_> = c.into_iter().collect();
    println!("{}", c.len());
    for &ci in &c {
        println!("{} {}", ci.0, ci.1);
    }
}
