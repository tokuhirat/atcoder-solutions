use proconio::{fastout, input};

fn is_p(k: usize) -> bool {
    let k: Vec<_> = k.to_string().chars().collect();
    let l = k.len();
    for i in 0..l {
        if k[i] != k[l - i - 1] {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut ans = 0;
    for i in 1..=n {
        let k = i * i * i;
        if k > n {
            break;
        }

        if is_p(k) {
            ans = k;
        }
    }
    println!("{}", ans);
}
