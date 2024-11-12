use proconio::{fastout, input, marker::Chars};
use std::cmp::Ordering;
#[fastout]
fn main() {
    input!(
        mut s: Chars,
        t: Chars,
    );
    let mut x = vec![];

    let mut ok = false;
    while !ok {
        ok = true;
        for i in 0..s.len() {
            match s[i].cmp(&t[i]) {
                Ordering::Less => {}
                Ordering::Equal => {}
                Ordering::Greater => {
                    s[i] = t[i];
                    x.push(s.clone());
                    ok = false
                }
            }
        }
    }
    while s != t {
        for i in (0..s.len()).rev() {
            if s[i].cmp(&t[i]) == Ordering::Less {
                s[i] = t[i];
                x.push(s.clone());
            }
        }
    }

    println!("{}", x.len());
    for xi in &x {
        for &xij in xi {
            print!("{}", xij);
        }
        println!("");
    }
}
