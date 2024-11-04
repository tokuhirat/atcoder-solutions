use proconio::{fastout, input};

fn update(dp: &mut [usize], size: usize, pos: usize, value: usize) {
    let mut pos = pos + size - 1;
    dp[pos] = value;
    while pos > 0 {
        pos = (pos - 1) / 2;
        dp[pos] = dp[2 * pos + 1].min(dp[2 * pos + 2]);
    }
}

fn query(dp: &mut [usize], l: usize, r: usize, pos: usize, a: usize, b: usize) -> usize {
    if b <= l || r <= a {
        return usize::MAX;
    }
    if l <= a && b <= r {
        return dp[pos];
    }
    let v1 = query(dp, l, r, 2 * pos + 1, a, (a + b) / 2);
    let v2 = query(dp, l, r, 2 * pos + 2, (a + b) / 2, b);
    v1.min(v2)
}

#[fastout]
fn main() {
    input!(
        n: usize,
        l: usize,
        r: usize,
        x: [usize; n],
    );
    let size = n.next_power_of_two();
    let mut dp = vec![usize::MAX; size * 2];
    update(&mut dp, size, 0, 0);
    for (i, &xi) in x.iter().enumerate().skip(1) {
        let left = x.partition_point(|&e| e + r < xi);
        let right = x.partition_point(|&e| e + l - 1 < xi);

        let cnt = query(&mut dp, left, right, 0, 0, size);
        if cnt < usize::MAX {
            update(&mut dp, size, i, cnt + 1);
        }
    }

    println!("{}", dp[(n - 1) + (size - 1)]);
}
