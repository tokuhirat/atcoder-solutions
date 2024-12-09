use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (_n, x, y, z): (usize, usize, usize, usize),
    );
    if (x < z && z < y) || (y < z && z < x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
