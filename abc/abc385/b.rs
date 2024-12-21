use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        (h, _w, mut x, mut y): (usize, usize, Usize1, Usize1),
        s: [Chars; h],
        t: Chars,
    );
    let mut home = HashSet::new();
    for &ti in &t {
        match ti {
            'U' => {
                if s[x - 1][y] == '#' {
                    continue;
                } else {
                    x -= 1;
                }
            }
            'D' => {
                if s[x + 1][y] == '#' {
                    continue;
                } else {
                    x += 1;
                }
            }
            'L' => {
                if s[x][y - 1] == '#' {
                    continue;
                } else {
                    y -= 1;
                }
            }
            'R' => {
                if s[x][y + 1] == '#' {
                    continue;
                } else {
                    y += 1;
                }
            }
            _ => unreachable!(),
        }
        if s[x][y] == '@' {
            home.insert((x, y));
        }
    }
    x += 1;
    y += 1;
    println!("{} {} {}", x, y, home.len());
}
