use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (h, w, _n): (usize, usize, usize),
        t: Chars,
        s: [Chars; h],
    };
    let mut ans = 0;
    for (i, si) in s.iter().enumerate().skip(1).take(h - 2) {
        'lo: for (j, &sij) in si.iter().enumerate().skip(1).take(w - 2) {
            if sij == '#' {
                continue;
            }
            let (mut i, mut j) = (i, j);
            for &ti in &t {
                match ti {
                    'L' => j -= 1,
                    'R' => j += 1,
                    'U' => i -= 1,
                    'D' => i += 1,
                    _ => unreachable!(),
                }
                if s[i][j] == '#' {
                    continue 'lo;
                }
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
