use proconio::{fastout, input};

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        return gcd(b, a);
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[fastout]
fn main() {
    input!(
        a: usize,
        b: usize,
    );

    let gcd_num = gcd(a, b);
    let lcm = a * b / gcd_num;
    println!("{}", lcm);
}
