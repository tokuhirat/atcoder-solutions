use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        (n, q): (usize, usize),
        mut s: Chars,
    );
    let mut set = HashSet::new();
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            set.insert(i as i32);
        }
    }

    for _ in 0..q {
        input!(
            (x, c): (Usize1, char)
        );
        if s[x] == c {
            println!("{}", set.len());
            continue;
        }
        s[x] = c;
        match c {
            'A' => {
                if set.contains(&(x as i32 - 1)) {
                    set.remove(&(x as i32 - 1));
                }
                if set.contains(&(x as i32 - 2)) {
                    set.remove(&(x as i32 - 2));
                }
                if x < n - 2 && s[x + 1] == 'B' && s[x + 2] == 'C' {
                    set.insert(x as i32);
                }
            }
            'B' => {
                if set.contains(&(x as i32)) {
                    set.remove(&(x as i32));
                }
                if set.contains(&(x as i32 - 2)) {
                    set.remove(&(x as i32 - 2));
                }
                if 0 < x && x < n - 1 && s[x - 1] == 'A' && s[x + 1] == 'C' {
                    set.insert(x as i32 - 1);
                }
            }
            'C' => {
                if set.contains(&(x as i32)) {
                    set.remove(&(x as i32));
                }
                if set.contains(&(x as i32 - 1)) {
                    set.remove(&(x as i32 - 1));
                }
                if 1 < x && s[x - 2] == 'A' && s[x - 1] == 'B' {
                    set.insert(x as i32 - 2);
                }
            }
            _ => {
                if set.contains(&(x as i32 - 2)) {
                    set.remove(&(x as i32 - 2));
                }
                if set.contains(&(x as i32 - 1)) {
                    set.remove(&(x as i32 - 1));
                }
                if set.contains(&(x as i32)) {
                    set.remove(&(x as i32));
                }
            }
        }
        println!("{}", set.len());
    }
}
