use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut a: [usize; n],
        b: [usize; n - 1],
    );
    a.sort_unstable();
    let mut cap = vec![0; n + 1];
    for bi in b.iter() {
        let pos = a.partition_point(|x| x <= bi);
        cap[pos] += 1;
    }

    let mut remain = cap[n];
    let mut ans = 0;
    for i in (0..n).rev() {
        if remain <= 0 && ans > 0 {
            println!("-1");
            return;
        }

        if remain <= 0 && ans == 0 {
            ans = a[i];
            remain += 1;
        }
        remain -= 1;
        remain += cap[i];
    }
    println!("{}", ans);
}
