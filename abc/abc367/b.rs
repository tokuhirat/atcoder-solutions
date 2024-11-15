use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        mut x: Chars,
    );
    for _ in 0..3 {
        if *x.last().unwrap() != '0' {
            let s: String = x.iter().collect();
            println!("{}", s);
            return;
        }
        x.pop();
    }
    x.pop();
    let s: String = x.iter().collect();
    println!("{}", s);
}
