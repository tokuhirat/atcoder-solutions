use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        ab: [(i64, i64); n],
    );

    let mut s1 = 0;
    let mut s2 = 0;
    let mut s3 = 0;
    let mut s4 = 0;
    for &(ai, bi) in ab.iter() {
        if ai + bi > 0 {
            s1 += ai + bi;
        }
        if ai - bi > 0 {
            s2 += ai - bi;
        }
        if bi - ai > 0 {
            s3 += bi - ai;
        }
        if -ai - bi > 0 {
            s4 += -ai - bi;
        }
    }
    let ans = s1.max(s2).max(s3).max(s4);
    println!("{}", ans);
}
