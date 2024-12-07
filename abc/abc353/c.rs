use proconio::{fastout, input};

const MOD: usize = 100000000;

#[fastout]
fn main() {
    input!(
        n: usize,
        mut a: [usize; n],
    );
    a.sort_unstable();
    let mut ans: usize = a.iter().sum();
    ans *= n - 1;

    for (i, &ai) in a.iter().enumerate() {
        let idx = a.partition_point(|&e| e + ai < MOD);
        if idx <= i {
            ans -= MOD * (n - i) * (n - i - 1) / 2;
            break;
        }
        ans -= MOD * (n - idx);
    }
    println!("{}", ans);
}
