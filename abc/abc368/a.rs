use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        a: [usize; n],
    );
    let start = n - k;
    let mut i = start;
    loop {
        print!("{}", a[i]);
        i += 1;
        i %= n;
        if i == start {
            println!("");
            break;
        } else {
            print!(" ");
        }
    }
}
