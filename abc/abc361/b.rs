use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (a, b, c, d, e, f): (usize, usize, usize, usize, usize, usize),
        (g, h, i, j, k, l): (usize, usize, usize, usize, usize, usize),
    );
    if j <= a || d <= g || k <= b || e <= h || l <= c || f <= i {
        println!("No");
    } else {
        println!("Yes");
    }
}
