use proconio::{fastout, input, marker::Chars};

const MOD: i64 = (1 << 31) - 1;

fn calc_hash(hash: &[i64], l: usize, r: usize, power: &[i64]) -> i64 {
    let mut h = hash[r] - (hash[l - 1] * power[r - l + 1]) % MOD;
    if h < 0 {
        h += MOD;
    }
    h
}

#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    );

    let s = s
        .iter()
        .map(|&x| (x as u8 - b'a' + 1) as i64)
        .collect::<Vec<_>>();

    let mut power = vec![1; n + 1];
    for i in 1..=n {
        power[i] = power[i - 1] * 100 % MOD;
    }

    let mut hash1 = vec![0; n + 1];
    let mut hash2 = vec![0; n + 1];

    for i in 1..=n {
        hash1[i] = (hash1[i - 1] * 100 % MOD + s[i - 1]) % MOD;
        hash2[i] = (hash2[i - 1] * 100 % MOD + s[n - i]) % MOD;
    }

    for &(l, r) in lr.iter() {
        let h1 = calc_hash(&hash1, l, r, &power);
        let h2 = calc_hash(&hash2, n - r + 1, n - l + 1, &power);
        if h1 == h2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
