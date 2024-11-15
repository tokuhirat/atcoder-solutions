use proconio::{fastout, input};

fn dfs(r: &[usize], i: usize, n: usize) -> Vec<Vec<usize>> {
    if i == n - 1 {
        let mut v = vec![];
        for i in 1..=r[i] {
            v.push(vec![i]);
        }
        return v;
    }
    let v = dfs(r, i + 1, n);
    let mut next_v = vec![];
    for j in 1..=r[i] {
        for vi in &v {
            next_v.push([vec![j], vi.clone()].concat());
        }
    }
    next_v
}

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        r: [usize; n],
    );
    let ans = dfs(&r, 0, n);
    for a in &ans {
        if a.iter().sum::<usize>() % k == 0 {
            for (i, ai) in a.iter().enumerate() {
                if i > 0 {
                    print!(" ")
                }
                print!("{}", ai);
            }
            println!("");
        }
    }
}
