use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        mut a: usize,
        mut b: usize
    );
    let m = 1000000007;
    println!("{}", power(&a, &b, &m));
}

fn power(a: &usize, b: &usize, m: &usize) -> usize {
    let mut ans = 1;
    let mut base = *a;
    let mut exp = *b;

    while exp > 0 {
        if exp & 1 == 1 {
            ans = (ans * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }
    ans
}
