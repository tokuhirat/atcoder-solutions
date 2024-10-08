use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    );

    let mut ans = 0;
    for a_min in 1..=100 - k {
        for b_min in 1..=100 - k {
            let mut temp = 0;
            for &(ai, bi) in ab.iter() {
                if a_min <= ai && ai <= a_min + k && b_min <= bi && bi <= b_min + k {
                    temp += 1;
                }
            }
            ans = ans.max(temp);
        }
    }
    println!("{}", ans);
}
