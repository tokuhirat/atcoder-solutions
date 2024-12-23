use proconio::{fastout, input};
use std::{collections::HashSet, mem::swap};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    );

    let mut s = HashSet::new();
    for &ai in &a {
        s.insert(ai);
    }

    let mut old = HashSet::new();
    swap(&mut old, &mut s);
    for &si in old.iter() {
        for &bi in &b {
            s.insert(si + bi);
        }
    }

    let mut old = HashSet::new();
    swap(&mut old, &mut s);
    for &si in old.iter() {
        for &ci in &c {
            s.insert(si + ci);
        }
    }

    for &xi in &x {
        if s.contains(&xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
