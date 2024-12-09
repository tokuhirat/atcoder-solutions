use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
    );
    let mut ca = Vec::with_capacity(m);
    for _ in 0..m {
        input!(
            (ki, ci): (usize, usize),
            ai: [Usize1; ki],
        );
        ca.push((ci, ai));
    }

    ca.sort_unstable();
    let mut uf = Dsu::new(n);
    let mut ans = 0;
    for (c, a) in &ca {
        let &a0 = a.first().unwrap();
        for &ai in a.iter().skip(1) {
            if uf.same(a0, ai) {
                continue;
            }
            uf.merge(a0, ai);
            ans += c;
        }
        if uf.size(0) == n {
            break;
        }
    }
    if uf.size(0) == n {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
