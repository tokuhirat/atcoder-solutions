use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    );
    let mut ans = 0;
    ans += a.iter().sum::<usize>() * c.len();
    ans += b * a.len() * c.len();
    ans += c.iter().sum::<usize>() * a.len();
    println!("{}", ans);
}
