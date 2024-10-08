use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        l: usize,
        ab: [(usize, char); n]
    );
    let mut left = l;
    let mut right = 0;
    for &(ai, bi) in ab.iter() {
        if bi == 'E' {
            left = ai;
            break;
        }
    }
    for &(ai, bi) in ab.iter().rev() {
        if bi == 'W' {
            right = ai;
            break;
        }
    }
    let ans = right.max(l - left);
    println!("{}", ans);
}
