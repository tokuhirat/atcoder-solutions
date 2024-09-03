use proconio::input;
fn main() {
    input!(n: i32, k: i32);
    let mut ans = 0;
    for i in 1..=n {
        if i >= k {
            break;
        }
        for j in 1..=n {
            if i + j >= k {
                break;
            }
            if k - i - j <= n {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
