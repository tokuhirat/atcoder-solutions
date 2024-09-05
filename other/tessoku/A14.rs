use proconio::input;

fn main() {
    input!(
        n: usize, k: i32,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
        d: [i32; n],
    );

    let mut p = vec![0; n * n];
    for (i, ai) in a.iter().enumerate().take(n) {
        for (j, bj) in b.iter().enumerate().take(n) {
            p[i * n + j] = ai + bj;
        }
    }

    let mut q = vec![0; n * n];
    for (i, ci) in c.iter().enumerate().take(n) {
        for (j, dj) in d.iter().enumerate().take(n) {
            q[i * n + j] = ci + dj;
        }
    }
    q.sort();

    let mut ans = String::from("No");
    for pi in p.iter() {
        let pos = q.partition_point(|&x| x + pi < k);
        if pos < n * n && pi + q[pos] == k {
            ans = String::from("Yes");
            break;
        }
    }

    println!("{}", ans);
}
