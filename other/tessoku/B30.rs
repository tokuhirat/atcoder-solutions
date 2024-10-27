use proconio::{fastout, input};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input!(
        h: usize,
        w: usize,
    );

    let n = h + w - 2;
    let r = h - 1;
    let (fac, finv) = combination_init(n);
    let ans = calc_combination(&fac, &finv, n, r);
    println!("{}", ans);
}

fn combination_init(size: usize) -> (Vec<usize>, Vec<usize>) {
    let mut fac = vec![1; size + 1];
    let mut finv = vec![1; size + 1];
    let mut inv = vec![1; size + 1];

    for i in 2..=size {
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }
    (fac, finv)
}

fn calc_combination(fac: &[usize], finv: &[usize], n: usize, r: usize) -> usize {
    if n < r {
        return 0;
    }
    fac[n] * (finv[r] * finv[n - r] % MOD) % MOD
}
