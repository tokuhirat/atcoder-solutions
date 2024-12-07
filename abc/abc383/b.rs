use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

fn count(s: &[Vec<char>], p1: &(usize, usize), p2: &(usize, usize), d: usize) -> usize {
    let mut ret = 0;
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '#' {
                continue;
            }
            if (i.abs_diff(p1.0) + j.abs_diff(p1.1) <= d)
                || i.abs_diff(p2.0) + j.abs_diff(p2.1) <= d
            {
                ret += 1;
            }
        }
    }
    ret
}

#[fastout]
fn main() {
    input!(
        (h, _w, d): (usize, usize, usize),
        s: [Chars; h],
    );
    let mut cand = vec![];
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == '.' {
                cand.push((i, j));
            }
        }
    }
    let mut ans = 0;
    for ab in cand.iter().combinations(2) {
        let p1 = ab[0];
        let p2 = ab[1];
        ans = ans.max(count(&s, p1, p2, d));
    }
    println!("{}", ans);
}
