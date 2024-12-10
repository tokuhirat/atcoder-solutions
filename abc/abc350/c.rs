use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut a: [Usize1; n],
    );
    let mut idx = vec![0; n];
    for (i, &ai) in a.iter().enumerate() {
        idx[ai] = i;
    }
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if idx[i] == i {
            continue;
        }
        let j = idx[i];
        ans.push((i + 1, j + 1));
        (idx[a[i]], idx[i]) = (idx[i], idx[a[i]]);
        (a[i], a[j]) = (a[j], a[i]);
    }
    println!("{}", ans.len());
    if !ans.is_empty() {
        for ansi in ans {
            println!("{} {}", ansi.0, ansi.1);
        }
    }
}
