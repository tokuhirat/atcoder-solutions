use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: [String; n],
    );
    let mut ng = false;
    if n <= 2 {
        println!("Yes");
        return;
    }
    for i in 0..n - 2 {
        if s[i] == "sweet" && s[i + 1] == "sweet" {
            ng = true
        }
    }
    if ng {
        println!("No");
    } else {
        println!("Yes");
    }
}
