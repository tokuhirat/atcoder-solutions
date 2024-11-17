use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: [Chars; n],
    );

    let m = s.iter().fold(0, |acc, x| acc.max(x.len()));
    let mut ans = vec![vec!['*'; n]; m];
    for (i, si) in s.iter().rev().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            ans[j][i] = sij;
        }
    }
    for ai in ans.iter_mut() {
        while let Some(&l) = ai.last() {
            if l == '*' {
                ai.pop();
            } else {
                break;
            }
        }
        for aij in ai {
            print!("{}", aij);
        }
        println!("");
    }
}
