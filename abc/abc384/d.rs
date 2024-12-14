use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, s): (usize, i64),
        a: [i64; n],
    );
    let a_sum: i64 = a.iter().sum();
    let s = s % a_sum;
    let a = [a.clone(), a].concat();

    let mut j = 0;
    let mut total = 0;
    for i in 0..n {
        if i > 0 {
            total -= a[i - 1];
        }
        while j < 2 * n && total < s {
            total += a[j];
            j += 1;
        }
        if total == s {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
