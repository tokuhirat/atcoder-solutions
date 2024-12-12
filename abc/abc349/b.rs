use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars
    );
    let mut cnt = vec![0; 101];
    for i in 0..26 {
        let c = s.iter().filter(|&si| *si as u8 == i + b'a').count();
        cnt[c] += 1;
    }

    for &i in cnt.iter().skip(1) {
        if i != 0 && i != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
