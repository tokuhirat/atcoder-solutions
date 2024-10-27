use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: [Chars; 8]
    );

    let mut table = vec![vec![true; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                for k in 0..8 {
                    table[i][k] = false;
                    table[k][j] = false;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if table[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
