use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: [usize; n],
    );

    let total: usize = k.iter().sum();
    let mut ans = usize::MAX;
    for i in 0..1 << n {
        let mut s = 0;
        for (j, &kj) in k.iter().enumerate() {
            if i & 1 << j != 0 {
                s += kj;
            }
        }
        ans = ans.min(s.max(total - s));
    }
    println!("{}", ans);
}
