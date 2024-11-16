use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        a: [usize; n],
    );
    let total = a.iter().sum::<usize>() % m;

    let mut cnt = vec![0; m];
    let mut cur = 0;
    let mut ans: usize = 0;
    for &ai in &a {
        ans += cnt[cur];
        ans += cnt[(m + cur - total) % m];
        cnt[cur] += 1;
        cur += ai;
        cur %= m;
    }
    println!("{}", ans);
}
