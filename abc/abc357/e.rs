use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [Usize1; n],
    );
    let mut used = vec![false; n];
    let mut index = vec![-1; n];
    let mut cnt = vec![0; n];
    let mut ord = vec![];
    for v in 0..n {
        if used[v] {
            continue;
        }
        let mut v = v;
        let mut path = vec![];
        loop {
            if used[v] {
                break;
            }
            if index[v] != -1 {
                let cycle = &path[index[v] as usize..];
                for &v in cycle {
                    cnt[v] = cycle.len();
                }
                break;
            }
            index[v] = path.len() as i32;
            path.push(v);
            v = a[v];
        }
        for &v in &path {
            used[v] = true;
        }
        for &v in path.iter().rev() {
            ord.push(v);
        }
    }
    for &v in &ord {
        if cnt[v] == 0 {
            cnt[v] = cnt[a[v]] + 1;
        }
    }
    let ans: usize = cnt.iter().sum();
    println!("{}", ans);
}
