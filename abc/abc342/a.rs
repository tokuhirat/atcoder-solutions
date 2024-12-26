use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut v = [0; 26];
    for &si in &s {
        v[(si as u8 - b'a') as usize] += 1;
    }
    let c = *v
        .iter()
        .enumerate()
        .filter(|(_i, &e)| e == 1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
        .first()
        .unwrap();
    let c = (b'a' + c as u8) as char;
    for (i, &si) in s.iter().enumerate() {
        if si == c {
            println!("{}", i + 1);
        }
    }
}
