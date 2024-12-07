use num_integer::Roots;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let sq = n.sqrt();
    let max = 2000000;
    let mut p = vec![true; max + 1];
    p[0] = false;
    p[1] = false;
    for i in 2..p.len() {
        if p[i] {
            let mut j = 2 * i;
            while j < p.len() {
                p[j] = false;
                j += i;
            }
        }
    }
    let mut prime = vec![];
    for (i, &pi) in p.iter().enumerate() {
        if pi {
            prime.push(i);
        }
    }

    let mut ans = 0;
    for (i, &pi) in prime.iter().enumerate() {
        let idx = prime.partition_point(|&x| x * pi <= sq);
        if idx <= i + 1 {
            break;
        }
        ans += idx - i - 1;
    }

    for &i in &prime {
        if i.pow(8) > n {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
