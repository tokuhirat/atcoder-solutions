use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        s: Chars
    );
    if s.len() % 2 == 1 {
        println!("No");
        return;
    }
    let mut a = HashSet::new();
    for i in 0..s.len() / 2 {
        if s[2 * i] != s[2 * i + 1] {
            println!("No");
            return;
        }
        if a.contains(&s[2 * i]) {
            println!("No");
            return;
        }
        a.insert(s[2 * i]);
    }
    println!("Yes");
}
