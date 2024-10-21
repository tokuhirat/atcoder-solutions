use proconio::{fastout, input};

fn to10(n: usize) -> usize {
    let mut n = n;
    let mut i = 0;
    let mut ret = 0;
    while n > 0 {
        if n & 1 == 1 {
            ret += 1 << i;
        }
        i += 1;
        n /= 10;
    }
    ret
}

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let ans = to10(n);
    println!("{}", ans);
}
