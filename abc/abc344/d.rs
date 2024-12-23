use proconio::{fastout, input, marker::Chars};
use std::{collections::HashMap, mem::swap};

#[fastout]
fn main() {
    input!(
        t: Chars,
        n: usize,
    );

    let mut s = HashMap::new();
    s.insert(0, 0);
    for _ in 0..n {
        input!(a: usize);
        let mut old = s.clone();
        swap(&mut old, &mut s);

        for _ in 0..a {
            input!(c: Chars);
            'outer: for (&idx, &p) in old.iter() {
                for (i, &ci) in c.iter().enumerate() {
                    if idx + i >= t.len() || t[idx + i] != ci {
                        continue 'outer;
                    }
                }
                let e = s.entry(idx + c.len()).or_insert(usize::MAX);
                *e = *e.min(&mut (p + 1));
            }
        }
    }
    if let Some(p) = s.get(&(t.len())) {
        println!("{}", p);
    } else {
        println!("-1");
    }
}
