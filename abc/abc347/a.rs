use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        a: [usize; n],
    );
    let mut ans = vec![];
    for &ai in &a {
        if ai % k == 0 {
            ans.push(ai / k);
        }
    }
    println!("{}", ans.iter().join(" "));
}
