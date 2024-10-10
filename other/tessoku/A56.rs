use proconio::{fastout, input, marker::Chars};

fn calc_hash(start: usize, end: usize, hash_vec: &[i64], power: &[i64], modulo: i64) -> i64 {
    let mut ret = hash_vec[end] - (hash_vec[start - 1] * power[end - start + 1] % modulo);
    if ret < 0 {
        ret += modulo;
    }
    ret
}

#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
        s: Chars,

    );
    let s = s
        .iter()
        .map(|&c| c as i64 - 'a' as i64 + 1)
        .collect::<Vec<i64>>();

    let modulo = (1 << 31) - 1;
    let p = 100;
    let mut power = vec![1; n + 1];
    for i in 1..=n {
        power[i] = (power[i - 1] * p) % modulo;
    }

    let mut hash_vec = vec![0; n + 1];
    for i in 1..=n {
        hash_vec[i] = (p * hash_vec[i - 1] % modulo + s[i - 1]) % modulo;
    }

    for _ in 0..q {
        input!(
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        );
        let h1 = calc_hash(a, b, &hash_vec, &power, modulo);
        let h2 = calc_hash(c, d, &hash_vec, &power, modulo);
        if h1 == h2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
