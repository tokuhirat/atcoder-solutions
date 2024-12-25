use itertools::iproduct;
use proconio::{fastout, input};

fn calc3(a1: usize, b1: usize, c1: usize, a2: usize, b2: usize, c2: usize) -> usize {
    let x_min = 7.max(a1).max(a2);
    let x_max = 14.min(a1 + 7).min(a2 + 7);
    let y_min = 7.max(b1).max(b2);
    let y_max = 14.min(b1 + 7).min(b2 + 7);
    let z_min = 7.max(c1).max(c2);
    let z_max = 14.min(c1 + 7).min(c2 + 7);
    x_max.saturating_sub(x_min) * y_max.saturating_sub(y_min) * z_max.saturating_sub(z_min)
}

fn calc2(a1: usize, b1: usize, c1: usize, a2: usize, b2: usize, c2: usize) -> usize {
    let x_min = a1.max(a2);
    let x_max = (a1 + 7).min(a2 + 7);
    let y_min = b1.max(b2);
    let y_max = (b1 + 7).min(b2 + 7);
    let z_min = c1.max(c2);
    let z_max = (c1 + 7).min(c2 + 7);
    x_max.saturating_sub(x_min) * y_max.saturating_sub(y_min) * z_max.saturating_sub(z_min)
}

#[fastout]
fn main() {
    input!(
        (v1, v2, v3): (usize, usize, usize),
    );
    for (a1, b1, c1) in iproduct!(0..=14, 0..=14, 0..=14) {
        for (a2, b2, c2) in iproduct!(0..=14, 0..=14, 0..=14) {
            let u3 = calc3(a1, b1, c1, a2, b2, c2);
            let u2 = calc2(a1, b1, c1, a2, b2, c2)
                + calc2(7, 7, 7, a2, b2, c2)
                + calc2(a1, b1, c1, 7, 7, 7)
                - 3 * u3;
            let u1 = 7 * 7 * 7 * 3 - u2 * 2 - u3 * 3;
            if u1 == v1 && u2 == v2 && u3 == v3 {
                println!("Yes");
                println!("7 7 7 {} {} {} {} {} {}", a1, b1, c1, a2, b2, c2);
                return;
            }
        }
    }
    println!("No");
}
