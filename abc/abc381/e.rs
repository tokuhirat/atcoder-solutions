use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

fn get_next(v: &[usize], i: usize, k: usize) -> usize {
    if k == 0 {
        return i;
    }
    let j = v.partition_point(|&e| e < i) + k - 1;
    if j < v.len() {
        v[j] + 1
    } else {
        usize::MAX
    }
}

fn f(len: usize, left: usize, right: usize, v1: &[usize], v2: &[usize], vs: &[usize]) -> bool {
    let i1 = get_next(v1, left, len);
    let is = get_next(vs, i1, 1);
    let i2 = get_next(v2, is, len);
    i2 <= right
}

#[fastout]
fn main() {
    input!(
        (n, q): (usize, usize),
        s: Chars
    );
    let mut v1 = Vec::with_capacity(n);
    let mut v2 = Vec::with_capacity(n);
    let mut vs = Vec::with_capacity(n);
    for (i, &si) in s.iter().enumerate() {
        match si {
            '1' => v1.push(i),
            '2' => v2.push(i),
            '/' => vs.push(i),
            _ => unreachable!(),
        }
    }

    for _ in 0..q {
        input!((l, r): (Usize1, usize));
        let mut len_ok: i32 = -1;
        let mut len_ng = (n + 1) as i32;
        while len_ok + 1 < len_ng {
            let mid = ((len_ok + len_ng) / 2) as usize;
            if f(mid, l, r, &v1, &v2, &vs) {
                len_ok = mid as i32
            } else {
                len_ng = mid as i32
            }
        }
        let ans = if len_ok == -1 { 0 } else { len_ok * 2 + 1 };
        println!("{}", ans);
    }
}
