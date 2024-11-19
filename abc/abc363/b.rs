use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, t, p): (usize, usize, usize),
        mut l: [usize; n],
    );
    l.sort_unstable();
    l.reverse();
    let target = l[p - 1];
    if target >= t {
        println!("0");
    } else {
        println!("{}", t - target);
    }
}
