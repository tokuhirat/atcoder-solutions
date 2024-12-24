use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [[usize; n]; n],
    );
    for ai in &a {
        let mut v = vec![];
        for (i, &aij) in ai.iter().enumerate() {
            if aij == 1 {
                v.push(i + 1);
            }
        }
        println!("{}", v.iter().join(" "));
    }
}
