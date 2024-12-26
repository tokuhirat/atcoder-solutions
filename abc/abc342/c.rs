use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
        q: usize,
    };

    let mut map: Vec<_> = (0..26).map(|i| (b'a' + i) as char).collect();

    let mut cnt = HashMap::new();
    for &si in &s {
        *cnt.entry(si).or_insert(0_usize) += 1;
    }

    for _ in 0..q {
        input! {(c, d): (char, char)};
        if !cnt.contains_key(&c) {
            continue;
        }
        if c == d {
            continue;
        }
        for mi in map.iter_mut() {
            if *mi == c {
                *mi = d;
            }
        }

        let c_cnt = cnt.entry(c).or_default();
        *cnt.entry(d).or_default() += *c_cnt;
        cnt.remove(&c);
    }

    for &si in &s {
        print!("{}", map[(si as u8 - b'a') as usize]);
    }
    println!("");
}
