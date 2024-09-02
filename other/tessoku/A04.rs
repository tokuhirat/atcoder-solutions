use proconio::input;
fn main() {
    input!(mut n: i32);
    let mut ans = String::new();
    while n > 0 {
        ans.push_str(&(n % 2).to_string());
        n /= 2;
    }
    ans = ans.chars().rev().collect();
    println!("{ans:0>10}");
}
