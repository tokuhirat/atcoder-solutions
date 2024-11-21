use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (xa, ya): (isize, isize),
        (xb, yb): (isize, isize),
        (xc, yc): (isize, isize),
    );
    let ab = xa.abs_diff(xb).pow(2) + ya.abs_diff(yb).pow(2);
    let bc = xb.abs_diff(xc).pow(2) + yb.abs_diff(yc).pow(2);
    let ca = xc.abs_diff(xa).pow(2) + yc.abs_diff(ya).pow(2);
    let mut v = [ab, bc, ca];
    v.sort_unstable();
    if v[0] + v[1] == v[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
