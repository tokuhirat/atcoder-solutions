use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
        q: usize,
    );
    let mut map = HashMap::new();
    map.insert(0, (0, *a.first().unwrap()));

    for i in 0..n - 1 {
        if i == 0 {
            map.insert(a[i], (0, a[1]));
        } else {
            map.insert(a[i], (a[i - 1], a[i + 1]));
        }
    }

    if n == 1 {
        map.insert(a[0], (0, usize::MAX));
    } else {
        map.insert(a[n - 1], (a[n - 2], usize::MAX));
    }

    map.insert(usize::MAX, (a[n - 1], usize::MAX));

    for _ in 0..q {
        input!(t: char);
        match t {
            '1' => {
                input!((x, y): (usize, usize));
                if let Some(&(bfr, nxt)) = map.get(&x) {
                    map.insert(x, (bfr, y));
                    map.insert(y, (x, nxt));
                    map.get_mut(&nxt).unwrap().0 = y;
                }
            }
            '2' => {
                input!(x: usize);
                if let Some(&(bfr, nxt)) = map.get(&x) {
                    map.get_mut(&bfr).unwrap().1 = nxt;
                    map.get_mut(&nxt).unwrap().0 = bfr;
                    map.remove(&x);
                }
            }
            _ => unreachable!(),
        };
    }
    let mut ans = vec![];
    let mut idx = 0;
    loop {
        let &(_, nxt) = map.get(&idx).unwrap();
        if nxt == usize::MAX {
            break;
        }
        ans.push(nxt);
        idx = nxt;
    }
    println!("{}", ans.iter().join(" "));
}
