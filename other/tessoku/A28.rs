use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let mut num = 0;

    for _ in 0..n {
        input!(
            t: char,
            a: i32
        );
        match t {
            '+' => num += a,
            '-' => num -= a,
            '*' => num *= a,
            _ => (),
        }
        if num < 0 {
            num += 10000;
        }
        num %= 10000;
        println!("{}", num);
    }
}
