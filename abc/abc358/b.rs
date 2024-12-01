use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, a): (usize, usize),
        t: [usize; n],
    );

    let mut now = 0;
    for &ti in &t {
        if now <= ti {
            now = ti + a;
        } else {
            now += a;
        }
        println!("{}", now);
    }
}
