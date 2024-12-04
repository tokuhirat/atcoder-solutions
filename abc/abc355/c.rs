use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        (n, t): (usize, usize),
        a: [Usize1; t],
    );
    let mut b = vec![0; 2 * n + 2];
    let mut ans = -1;
    for (i, &ai) in a.iter().enumerate() {
        let r = ai / n;
        let c = ai % n;
        b[r] += 1;
        b[n + c] += 1;
        if r == c {
            b[2 * n] += 1;
        }
        if r + c == n - 1 {
            b[2 * n + 1] += 1;
        }
        if b[r] == n || b[n + c] == n || b[2 * n] == n || b[2 * n + 1] == n {
            ans = i as i32 + 1;
            break;
        }
    }
    println!("{}", ans);
}
