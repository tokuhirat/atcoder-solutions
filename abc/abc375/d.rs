use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
    );

    let mut c_from_left = vec![vec![0; 26]; s.len() + 1];
    for (i, &si) in s.iter().enumerate() {
        for ci in 0..26 {
            c_from_left[i + 1][ci] = c_from_left[i][ci];
        }
        c_from_left[i + 1][si as usize - 'A' as usize] += 1;
    }

    let mut c_from_right = vec![vec![0; 26]; s.len() + 1];
    for (i, &si) in s.iter().rev().enumerate() {
        for ci in 0..26 {
            c_from_right[i + 1][ci] = c_from_right[i][ci];
        }
        c_from_right[i + 1][si as usize - 'A' as usize] += 1;
    }

    if s.len() < 3 {
        println!("0");
        return;
    }

    let mut ans: usize = 0;
    for i in 1..s.len() - 1 {
        for ci in 0..26 {
            ans += c_from_left[i][ci] * c_from_right[s.len() - i - 1][ci];
        }
    }
    println!("{}", ans);
}
