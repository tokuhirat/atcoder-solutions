#![allow(clippy::needless_range_loop)]
use proconio::{fastout, input};

fn f(x: usize) -> usize {
    let x: Vec<_> = x
        .to_string()
        .chars()
        .map(|e| (e as u8 - b'0') as usize)
        .collect();
    let mut ret = 0;

    if x.iter().skip(1).all(|xi| *xi < x[0]) {
        ret += 1
    }

    for i in 1..x.len() {
        ret += x[0].pow((x.len() - i - 1) as u32) * x[0].min(x[i]);
        if x[i] >= x[0] {
            break;
        }
    }

    for i in 1..x[0] {
        ret += i.pow((x.len() - 1) as u32);
    }

    for i in 1..x.len() {
        for j in 1..10_usize {
            ret += j.pow(i as u32 - 1);
        }
    }

    ret
}

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let ans = f(r) - f(l - 1);
    println!("{}", ans);
}
