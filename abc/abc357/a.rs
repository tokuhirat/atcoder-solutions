use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, mut m): (usize, usize),
        h: [usize; n],
    );
    let mut ans = 0;
    for &hi in &h {
        if hi <= m {
            m -= hi;
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
