use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (mut a, mut b): (usize, usize),
    );
    if a > b {
        (a, b) = (b, a);
    }
    let ans = match (a, b) {
        (1, 1) => -1,
        (1, 2) => 3,
        (1, 3) => 2,
        (2, 2) => -1,
        (2, 3) => 1,
        (3, 3) => -1,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
