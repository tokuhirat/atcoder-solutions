use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let b: Vec<usize> = a
        .iter()
        .map(|ai| 10_usize.pow(ai.to_string().len() as u32))
        .collect();
    let mut b_cum = Vec::with_capacity(n + 1);
    b_cum.push(0);
    for &bi in &b {
        b_cum.push((b_cum.last().unwrap() + bi) % 998244353);
    }
    let mut ans = ModInt998244353::new(0);
    for (i, &ai) in a.iter().enumerate() {
        ans += ai * (i + (b_cum[n] - b_cum[i + 1] + 998244353) % 998244353);
    }
    println!("{}", ans);
}
