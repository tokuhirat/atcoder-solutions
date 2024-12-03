use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        (n, l, r): (usize, Usize1, Usize1),
    );
    let a: Vec<usize> = (1..=n).collect();
    let mut ans = vec![];
    for (i, &ai) in a.iter().enumerate() {
        if l <= i && i <= r {
            ans.push(a[r + l - i]);
        } else {
            ans.push(ai);
        }
    }
    println!("{}", ans.iter().join(" "));
}
