use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        _h: usize,
        _w: usize,
        ab: [(usize, usize); n]
    );

    let mut v = vec![];
    for &(ai, bi) in ab.iter() {
        v.push(ai - 1);
        v.push(bi - 1);
    }
    let xor_sum = v.iter().fold(0, |acc, e| acc ^ e);
    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
