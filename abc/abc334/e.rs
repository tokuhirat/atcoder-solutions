use std::collections::HashSet;

use ac_library::{Dsu, ModInt998244353 as Mint};
use proconio::{fastout, input, marker::Chars};

const DIR: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    };
    let mut uf = Dsu::new(h * w);
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '#' {
                for &(di, dj) in &DIR {
                    let ni = i.wrapping_add_signed(di);
                    let nj = j.wrapping_add_signed(dj);
                    if ni >= h || nj >= w {
                        continue;
                    }
                    if s[ni][nj] == '#' {
                        uf.merge(i * w + j, ni * w + nj);
                    }
                }
            }
        }
    }

    let mut seen = HashSet::new();
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '#' {
                seen.insert(uf.leader(i * w + j));
            }
        }
    }
    let size = seen.len();

    let mut num: usize = 0;
    let mut cnt: usize = 0;
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '.' {
                num += 1;
                let mut seen = HashSet::new();
                for &(di, dj) in &DIR {
                    let ni = i.wrapping_add_signed(di);
                    let nj = j.wrapping_add_signed(dj);
                    if ni >= h || nj >= w {
                        continue;
                    }
                    if s[ni][nj] == '#' {
                        seen.insert(uf.leader(ni * w + nj));
                    }
                }
                match seen.len() {
                    0 => cnt += size + 1,
                    1 => cnt += size,
                    _ => cnt += size - (seen.len() - 1),
                }
            }
        }
    }

    let ans = Mint::from(cnt) / num;
    println!("{}", ans);
}
