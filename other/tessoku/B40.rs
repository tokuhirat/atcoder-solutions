use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let mut res = vec![0; 100];
    for &ai in a.iter() {
        res[ai % 100] += 1;
    }

    let mut ans = 0;
    for i in 1..=49 {
        ans += res[i] * res[100 - i];
    }
    if res[0] > 1 {
        ans += res[0] * (res[0] - 1) / 2;
    }
    if res[50] > 1 {
        ans += res[50] * (res[50] - 1) / 2;
    }
    println!("{}", ans);
}
