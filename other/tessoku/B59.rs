use proconio::{fastout, input};

fn update(segtree: &mut [usize], size: usize, pos: usize, value: usize) {
    let mut pos = pos + size - 1;
    segtree[pos] = value;
    while pos > 0 {
        pos = (pos - 1) / 2;
        segtree[pos] = segtree[2 * pos + 1] + segtree[2 * pos + 2];
    }
}

fn query(
    segtree: &mut [usize],
    left: usize,
    right: usize,
    pos: usize,
    a: usize,
    b: usize,
) -> usize {
    if b <= left || right <= a {
        return 0;
    }
    if left <= a && b <= right {
        return segtree[pos];
    }
    let v1 = query(segtree, left, right, 2 * pos + 1, a, (a + b) / 2);
    let v2 = query(segtree, left, right, 2 * pos + 2, (a + b) / 2, b);
    v1 + v2
}

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let size = n.next_power_of_two();
    let mut segtree = vec![0; size * 2];
    let mut ans = 0;
    for &ai in a.iter() {
        ans += query(&mut segtree, ai - 1, n, 0, 0, size);
        update(&mut segtree, size, ai - 1, 1);
    }
    println!("{}", ans);
}
