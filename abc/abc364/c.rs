use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, mut x, mut y): (usize, usize, usize),
        mut a: [usize; n],
        mut b: [usize; n],
    );
    a.sort_unstable();
    a.reverse();
    b.sort_unstable();
    b.reverse();
    let mut ans = 0;
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        ans += 1;
        if ai > x {
            break;
        } else {
            x -= ai;
        }
        if bi > y {
            break;
        } else {
            y -= bi;
        }
    }
    println!("{}", ans);
}
