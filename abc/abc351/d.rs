use ac_library::Dsu;
use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;
#[fastout]
fn main() {
    input!(
        (h, w): (usize, usize),
        s: [Chars; h],
    );
    let direction = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    // let mut mag = HashSet::new();
    let mut end = HashSet::new();
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '#' {
                // mag.insert((i, j));
                for d in direction {
                    let x = i as i32 + d.0;
                    let y = j as i32 + d.1;
                    if x < 0 || h as i32 <= x || y < 0 || w as i32 <= y {
                        continue;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    end.insert((x, y));
                }
            }
        }
    }
    let mut uf = Dsu::new(h * w * 4);
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '#' || end.contains(&(i, j)) {
                continue;
            }

            for &d in &direction {
                let x = i as i32 + d.0;
                let y = j as i32 + d.1;
                if x < 0 || h as i32 <= x || y < 0 || w as i32 <= y {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if s[x][y] == '#' || end.contains(&(x, y)) {
                    continue;
                }
                uf.merge(i * w + j, x * w + y);
            }
        }
    }
    for (i, si) in s.iter().enumerate() {
        for (j, _) in si.iter().enumerate() {
            if !end.contains(&(i, j)) {
                continue;
            }
            for (di, &d) in direction.iter().enumerate() {
                let x = i as i32 + d.0;
                let y = j as i32 + d.1;
                if x < 0 || h as i32 <= x || y < 0 || w as i32 <= y {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if s[x][y] == '#' || end.contains(&(x, y)) {
                    continue;
                }

                let mut same = false;
                for dj in 0..3 {
                    if dj == di {
                        continue;
                    }
                    if uf.same(h * w * dj + i * w + j, x * w + y) {
                        same = true;
                    }
                }
                if !same {
                    uf.merge(h * w * di + i * w + j, x * w + y);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..h * w {
        ans = ans.max(uf.size(i));
    }
    println!("{}", ans);
}
