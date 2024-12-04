use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut lr = Vec::with_capacity(n * 2);
    for _ in 0..n {
        input!((li, ri): (usize, usize));
        lr.push((li, 1));
        lr.push((ri + 1, -1));
    }
    lr.sort_unstable();
    let mut cur: usize = 0;
    let mut ans = 0;
    for &(_, t) in &lr {
        if t == 1 {
            ans += cur;
            cur += 1;
        } else {
            cur -= 1;
        }
    }
    println!("{}", ans);
}
