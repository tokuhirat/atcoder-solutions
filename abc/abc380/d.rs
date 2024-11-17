use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input!(
        s: Chars,
        q: usize,
    );
    let mut size = s.len();
    for i in 0..q {
        input!(
            mut k: Usize1
        );

        while size <= k {
            size *= 2;
        }
        let mut cnt = 0;
        while k >= s.len() {
            if 2 * k >= size {
                cnt += 1;
            }
            size /= 2;
            k %= size;
        }

        if cnt % 2 == 0 {
            print!("{}", s[k]);
        } else if s[k].is_ascii_lowercase() {
            print!("{}", s[k].to_uppercase());
        } else {
            print!("{}", s[k].to_lowercase());
        }
        if i != q - 1 {
            print!(" ");
        }
    }
    println!("");
}
