use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let mut prime = vec![true; n + 1];
    prime[0] = false;
    prime[1] = false;
    for i in 2..=n {
        if prime[i] {
            let mut j = 2 * i;
            while j <= n {
                prime[j] = false;
                j += i;
            }
        }
    }
    for (i, &b) in prime.iter().enumerate() {
        if b {
            println!("{}", i);
        }
    }
}
