use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        h: u32,
    );
    let mut i = 0;
    let mut len = 0;
    while len + 2_u32.pow(i) <= h {
        len += 2_u32.pow(i);
        i += 1;
    }
    println!("{}", i + 1);
}
