use proconio::input;

fn main() {
    input!(
        n: usize,
        q: usize,
    );

    let mut size = 1;
    while size < n {
        size *= 2;
    }
    let mut tree = vec![0; 2 * size - 1];

    fn update(tree: &mut [usize], size: usize, mut pos: usize, x: usize) {
        pos += size - 1;
        tree[pos] = x;
        while pos > 0 {
            pos = (pos - 1) / 2;
            tree[pos] = tree[2 * pos + 1].max(tree[2 * pos + 2]);
        }
    }

    fn max(tree: &mut [usize], size: usize, l: usize, r: usize) -> usize {
        max_(tree, l, r, 0, 0, size)
    }

    fn max_(tree: &mut [usize], l: usize, r: usize, pos: usize, a: usize, b: usize) -> usize {
        if b <= l || r <= a {
            0
        } else if l <= a && b <= r {
            tree[pos]
        } else {
            let v1 = max_(tree, l, r, 2 * pos + 1, a, (a + b) / 2);
            let v2 = max_(tree, l, r, 2 * pos + 2, (a + b) / 2, b);
            v1.max(v2)
        }
    }

    for _ in 0..q {
        input!(c: u8);
        if c == 1 {
            input!(pos: usize, x: usize);
            update(&mut tree, size, pos - 1, x);
        } else {
            input!(l: usize, r: usize);
            let ans = max(&mut tree, size, l - 1, r - 1);
            println!("{}", ans);
        }
    }
}
