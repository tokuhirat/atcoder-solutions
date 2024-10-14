use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
    );

    let n = n.next_power_of_two();
    let mut tree = vec![0; 2 * n - 1];

    fn update(tree: &mut [usize], n: usize, pos: usize, x: usize) {
        let mut pos = pos + n - 1;
        let old_x = tree[pos];
        tree[pos] = x;

        while pos > 0 {
            pos = (pos - 1) / 2;
            tree[pos] += x - old_x;
        }
    }

    fn query(tree: &mut [usize], n: usize, left: usize, right: usize) -> usize {
        query_sub(tree, left, right, 0, 0, n)
    }

    fn query_sub(
        tree: &mut [usize],
        left: usize,
        right: usize,
        pos: usize,
        a: usize,
        b: usize,
    ) -> usize {
        if b <= left || right <= a {
            0
        } else if left <= a && b <= right {
            tree[pos]
        } else {
            let v1 = query_sub(tree, left, right, 2 * pos + 1, a, (a + b) / 2);
            let v2 = query_sub(tree, left, right, 2 * pos + 2, (a + b) / 2, b);
            v1 + v2
        }
    }

    for _ in 0..q {
        input!(t: usize);
        if t == 1 {
            input!(pos: usize, x: usize);
            update(&mut tree, n, pos - 1, x);
        } else {
            input!(l: usize, r: usize);
            let ans = query(&mut tree, n, l - 1, r - 1);
            println!("{}", ans);
        }
    }
}
